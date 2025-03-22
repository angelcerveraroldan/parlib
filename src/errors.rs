pub mod pretty_error;
pub mod simple_error;

#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    PatternNotFound(String),
    CannotParseAnEmptyString,
    /// Parsing worked, by could not map
    MappingError(String),
    /// A custom error that can be added to a parser
    CustomError(String),
}
