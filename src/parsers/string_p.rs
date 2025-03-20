use crate::{inputs::Input, traits::Parser};

use super::{ParseMatch, ParseWhile};

pub struct StringParser;

impl Parser for StringParser {
    type Output = String;
    fn parse(&self, input: &Input) -> crate::type_alias::ParserRes<Self::Output> {
        // First, we will make sure that the first character is "
        let (_, rest) = ParseMatch('"').parse(input)?;
        let mut acc = 1;
        let mut chars = rest.source.chars().peekable();
        loop {
            match chars.next() {
                Some('"') => {
                    acc += 1;
                    break;
                }
                Some('\\') => {
                    let _ = chars.next().unwrap();
                    acc += 2;
                }
                Some(_) => acc += 1,
                None => {
                    return Err(crate::errors::ParsingError::PatternNotFound(
                        "Did not find closing quote \"".to_string(),
                    ))
                }
            };
        }

        Ok((
            input.source.chars().take(acc).collect(),
            input.clone().char_offset(acc),
        ))
    }
}

pub fn string_parser() -> impl Parser<Output = String> {
    StringParser
}

#[cfg(test)]
mod string_parser_test {
    use crate::parsers::string_p::string_parser;
    use crate::traits::Parser;

    #[test]
    fn string_parser_test() {
        let sp = string_parser();
        let (p, inp) = sp
            .parse(&"\"This is some string\" and this is the rest".into())
            .unwrap();
        assert_eq!(p, "This is some string".to_string());
        assert_eq!(inp.source, " and this is the rest".to_string());
        assert_eq!(inp.col, 23);
    }
}
