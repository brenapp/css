/**
 * Commerical At
 *
 * Can Be:
 *   At Keyword Token
 *   Delimiter
 *
 */
use super::super::check::{is_identifier, next_char_equals};
use super::super::consume;
use super::super::error::ParseError;
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if next_char_equals(points, &'@') {
        *position += 1;
        points.next();

        if is_identifier(points) {
            match consume::name(points, position) {
                Ok(name) => Ok(Some(CSSToken::AtKeyword(name))),
                Err(e) => Err(e),
            }
        } else {
            *position += 1;
            let next = points.next();

            match next {
                Some(c) => Ok(Some(CSSToken::Delim(c))),
                None => Err(ParseError {
                    at: *position,
                    token: None,
                    error_text: "Unexpected End of File (EoF) around @",
                }),
            }
        }
    } else {
        Ok(None)
    }
}
