use nom::IResult;
use nom;

pub fn u64_to_u32(a: u64) -> u32 { a as u32 }

pub fn flat_map<I, O, E, S, F>(r: IResult<I, O, E>, f: F) -> IResult<I, S, E>
    where F: FnOnce(I, O) -> IResult<I, S, E> {
    match r {
        IResult::Done(i, o) => f(i, o),
        IResult::Error(e) => IResult::Error(e),
        IResult::Incomplete(e) => IResult::Incomplete(e),
    }
}

pub fn map_err(input: nom::ErrorKind<u32>) -> nom::Err<super::error::Error> {
    use nom::ErrorKind::*;
    match input {
        Custom(e) => Custom(super::error::Error::Other(e)),
        Tag => Tag,
        MapRes => MapRes,
        MapOpt => MapOpt,
        Alt => Alt,
        IsNot => IsNot,
        IsA => IsA,
        SeparatedList => SeparatedList,
        SeparatedNonEmptyList => SeparatedNonEmptyList,
        Many0 => Many0,
        Many1 => Many1,
        ManyTill => ManyTill,
        Count => Count,
        TakeUntilAndConsume => TakeUntilAndConsume,
        TakeUntil => TakeUntil,
        TakeUntilEitherAndConsume => TakeUntilEitherAndConsume,
        TakeUntilEither => TakeUntilEither,
        LengthValue => LengthValue,
        TagClosure => TagClosure,
        Alpha => Alpha,
        Digit => Digit,
        HexDigit => HexDigit,
        OctDigit => OctDigit,
        AlphaNumeric => AlphaNumeric,
        Space => Space,
        MultiSpace => MultiSpace,
        LengthValueFn => LengthValueFn,
        Eof => Eof,
        ExprOpt => ExprOpt,
        ExprRes => ExprRes,
        CondReduce => CondReduce,
        Switch => Switch,
        TagBits => TagBits,
        OneOf => OneOf,
        NoneOf => NoneOf,
        Char => Char,
        CrLf => CrLf,
        RegexpMatch => RegexpMatch,
        RegexpMatches => RegexpMatches,
        RegexpFind => RegexpFind,
        RegexpCapture => RegexpCapture,
        RegexpCaptures => RegexpCaptures,
        TakeWhile1 => TakeWhile1,
        Complete => Complete,
        Fix => Fix,
        Escaped => Escaped,
        EscapedTransform => EscapedTransform,
        TagStr => TagStr,
        IsNotStr => IsNotStr,
        IsAStr => IsAStr,
        TakeWhile1Str => TakeWhile1Str,
        NonEmpty => NonEmpty,
        ManyMN => ManyMN,
        TakeUntilAndConsumeStr => TakeUntilAndConsumeStr,
        TakeUntilStr => TakeUntilStr,
        Not => Not,
        Permutation => Permutation,
        Verify => Verify,
    }
}
