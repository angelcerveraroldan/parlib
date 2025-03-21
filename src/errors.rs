use crate::traits::Parser;

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    kind: ParsingErrorKind,
    line: usize,
    col: usize,
}

impl PartialOrd for ParsingError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.line > other.line {
            Some(std::cmp::Ordering::Greater)
        } else if self.line < other.line {
            Some(std::cmp::Ordering::Less)
        }
        // Now we know that the line number is the same
        else if self.col > other.col {
            Some(std::cmp::Ordering::Greater)
        } else if self.col < other.col {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl ParsingError {
    pub fn new(kind: ParsingErrorKind, line: usize, col: usize) -> Self {
        ParsingError { kind, line, col }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    PatternNotFound(String),
    CannotParseAnEmptyString,
    MappingError(String),
    /// A custom error that can be added to a parser
    CustomError(String),
}

/// Add a custom error message to some parser
pub struct ErrorParser<'a, P>
where
    P: Parser,
{
    parser: P,
    message: &'a str,
}

impl<'a, P> ErrorParser<'a, P>
where
    P: Parser,
{
    pub fn new(parser: P, message: &'a str) -> Self {
        ErrorParser { parser, message }
    }
}

impl<'a, P> Parser for ErrorParser<'a, P>
where
    P: Parser,
{
    type Output = P::Output;

    fn parse(&self, input: &crate::inputs::Input) -> crate::type_alias::ParserRes<Self::Output> {
        self.parser.parse(input).map_err(|err| {
            let kind = ParsingErrorKind::CustomError(self.message.to_string());
            ParsingError::new(kind, err.line, err.col)
        })
    }
}
