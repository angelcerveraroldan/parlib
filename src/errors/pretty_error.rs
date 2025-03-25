use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{traits::Parser, MainParser};

use super::simple_error::ParsingError;

/// A pretty error to report where along parsing the error occurred
///
/// This needs acess to the entire source code in order to display a
/// nice error message, so it should only be used at the 'root parser'
#[derive(Debug, Error, Diagnostic)]
#[error("Error during parsing")]
#[diagnostic(help("try doing it better next time?"))]
pub struct PrettyError {
    #[source_code]
    src: String,
    #[label("Parsing Error Here")]
    position: SourceSpan,
}

impl<P: Parser> From<(ParsingError, &MainParser<P>)> for PrettyError {
    fn from((perror, pmain): (ParsingError, &MainParser<P>)) -> Self {
        Self {
            src: pmain.src.clone(),
            position: (perror.line, perror.col).into(),
        }
    }
}
