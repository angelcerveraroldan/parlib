use crate::{errors::ParsingError, inputs::Input};

pub type ParserRes<A, E = ParsingError> = std::result::Result<(A, Input), E>;
