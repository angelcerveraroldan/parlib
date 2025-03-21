use crate::traits::Parser;

#[derive(Debug, PartialEq)]
pub enum ParsingError {
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
        self.parser
            .parse(input)
            .map_err(|_| ParsingError::CustomError(self.message.to_string()))
    }
}
