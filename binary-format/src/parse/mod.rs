use nom;

mod leb128;
mod uints;
mod util;

pub mod error;

use self::error::Error as CustomError;

fn take_bytes<'a>(input: &'a[u8], n: usize) -> nom::IResult<&'a[u8], &'a[u8], CustomError> {
    take!(input, n).map_err(util::map_err)
}

fn tag<'a>(input: &'a[u8], s: &'a [u8]) -> nom::IResult<&'a[u8], &'a[u8], CustomError> {
    tag!(input, s).map_err(util::map_err)
}

pub struct Module<'a> {
    pub version_number: u32,
    pub sections: Vec<SectionType<'a>>
}

pub enum SectionType<'a> {
    TypeSection(Vec<func_type::FuncType>),
    ImportSection(Vec<import_entry::ImportEntry<'a>>),
}

// Module Preamble
pub mod module {
    use super::uints;
    use super::Module;
    use super::error::Error as CustomError;

    named!(magic_number<&[u8], &[u8], CustomError>, call!(super::tag, b"\0asm"));
    named!(version_number<&[u8], u32, CustomError>, call!(uints::parse_u32));
    named!(pub parse<&[u8], Module, CustomError>,
           do_parse!(
               call!(magic_number) >>
                   vn: call!(version_number) >>
                   (Module {
                       version_number: vn,
                       sections: vec![],
                   })
           ));
}

pub fn parse_value_type<'a>(input: &'a[u8]) -> nom::IResult<&'a[u8], ::wasm_ast::types::ValueType, CustomError> {
    util::flat_map(leb128::parse_varuint32(input), |input, value| {
        let ok = match value as u8 {
            0x7f => ::wasm_ast::types::ValueType::I32,
            0x7e => ::wasm_ast::types::ValueType::I64,
            0x7d => ::wasm_ast::types::ValueType::F32,
            0x7c => ::wasm_ast::types::ValueType::F64,
            _ => return nom::IResult::Error(nom::ErrorKind::Custom(CustomError::InvalidValueType(value)))
        };
        nom::IResult::Done(input, ok)
    })
}

pub mod section {
    use super::leb128;
    use super::util::{self, map_err};
    use super::error::Error as CustomError;
    use nom::IResult;

    pub struct Section<'a> {
        pub id: u32,
        pub name: Option<&'a [u8]>,
        pub data: &'a [u8],
    }


    // Sections
    named!(section_id<&[u8], u32, CustomError>, map!(leb128::parse_varuint7, util::u64_to_u32));
    named!(section_payload_len<&[u8], u64, CustomError>, call!(leb128::parse_varuint32));
    named!(section_name_len<&[u8], u64, CustomError>, call!(leb128::parse_varuint32));
    fn section_name<'a>(input: &'a[u8], len: u64) -> IResult<&'a[u8], &'a[u8], CustomError> {
        take!(input, len).map_err(map_err)
    }
    fn section_payload_data<'a>(input: &'a[u8], len: u64) -> IResult<&'a[u8], &'a[u8], CustomError> {
        take!(input, len).map_err(map_err)
    }

    fn maybe_parse_name<'a>(input: &'a[u8], is_zero: bool) -> IResult<&'a[u8], Option<&'a[u8]>, CustomError> {
        cond!(input, is_zero, do_parse!(
            len: call!(section_name_len) >>
            name: call!(section_name, len) >>
            (name)
        ))
    }

    pub fn parse_section<'a>(input: &'a[u8]) -> IResult<&'a [u8], Section, CustomError> {
        do_parse!(input,
                  section_id: call!(section_id) >>
                  section_payload_len: call!(section_payload_len) >>
                  maybe_name: call!(maybe_parse_name, section_id == 0) >>
                  data: call!(section_payload_data, section_payload_len) >>
                  (Section {
                      id: section_id,
                      name: maybe_name,
                      data: data
                  })
        )
    }
}

pub mod func_type {
    use wasm_ast::types::ValueType;
    use nom::IResult;
    use super::leb128;
    use super::parse_value_type;
    use super::util::map_err;
    use super::error::Error as CustomError;

    pub struct FuncType {
        // TODO(tyoverby): what is a form?
        pub form: i64,
        pub param_types: Vec<ValueType>,
        pub return_type: Option<ValueType>,
    }

    named!(function_type_form<&[u8], i64, CustomError>, call!(leb128::parse_varint7));
    named!(function_type_param_count<&[u8], u64, CustomError> , call!(leb128::parse_varuint32));
    fn function_types<'a>(input: &'a[u8], length: u64) -> IResult<&'a[u8], Vec<ValueType>, CustomError> {
        count!(input, parse_value_type, length as usize).map_err(map_err)
    }
    named!(function_return_count<&[u8], u64, CustomError>, call!(leb128::parse_varuint1));

    pub fn parse<'a>(input: &'a[u8]) -> IResult<&'a[u8], FuncType, CustomError> {
        do_parse!(
            input,
            form: call!(function_type_form) >>
            arg_count: call!(function_type_param_count) >>
            args: call!(function_types, arg_count) >>
            ret_count: call!(function_return_count) >>
            ret_values: cond!(ret_count == 1, call!(parse_value_type)) >>
            (FuncType {
                form: form,
                param_types: args,
                return_type: ret_values
            })
        )
    }
}

pub mod import_entry {
    use nom::IResult;
    use super::error::Error as CustomError;
    use super::leb128;
    use super::uints;
    use super::take_bytes;

    pub enum ExternalKind {
        Function, Table, Memory, Global,
    }
    pub struct ImportEntry<'a> {
        pub module: &'a[u8],
        pub field: &'a[u8],
        pub external_kind: ExternalKind,
    }

    pub fn parse<'a>(input: &'a[u8]) -> IResult<&'a[u8], ImportEntry<'a>, CustomError> {
        fn map_external_kind(value: u8) -> Result<ExternalKind, CustomError> {
            let ok = match value {
                0 => ExternalKind::Function,
                1 => ExternalKind::Table,
                2 => ExternalKind::Memory,
                3 => ExternalKind::Global,
                _ => return Result::Err(CustomError::InvalidExternalKind(value)),
            };

            Ok(ok)
        }

        do_parse!(input,
            mod_len: call!(leb128::parse_varuint32) >>
            mod_bytes: call!(take_bytes, mod_len as usize) >>
            field_len: call!(leb128::parse_varuint32) >>
            field_bytes: call!(take_bytes, field_len as usize) >>
            ext_knd: map_res!(call!(uints::parse_u8), map_external_kind) >>
            (ImportEntry {
                module: mod_bytes,
                field: field_bytes,
                external_kind: ext_knd,
            })
        )
    }
}

pub mod type_section {
    use nom::IResult;
    use super::error::Error as CustomError;
    use super::leb128;
    use super::func_type;
    use super::SectionType;

    pub fn parse_data<'a>(input: &'a[u8]) -> IResult<&'a[u8], SectionType, CustomError> {
        do_parse!(
            input,
            len: call!(leb128::parse_varuint32) >>
            types: count!(call!(func_type::parse), len as usize) >>
            (SectionType::TypeSection(types))
        )
    }
}

pub mod import_section {
    use nom::IResult;
    use super::error::Error as CustomError;
    use super::leb128;
    use super::import_entry;
    use super::SectionType;

    pub fn parse_data<'a>(input: &'a[u8]) -> IResult<&'a[u8], SectionType, CustomError> {
        do_parse!(
            input,
            len: call!(leb128::parse_varuint32) >>
            imports: count!(call!(import_entry::parse), len as usize) >>
            (SectionType::ImportSection(imports))
        )
    }
}

pub fn parse(_bytes: &[u8]) {


    unimplemented!();
}
