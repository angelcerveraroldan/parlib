use crate::{errors::simple_error::ParsingError, inputs::Input};

pub type ParserRes<A, E = ParsingError> = std::result::Result<(A, Input), E>;
