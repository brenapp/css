/**
 * Parses CSS Hash tokens (#id, or #FFFFFF)
 *
 * Possible outcomes:
 *  - Ok(None) => No match
 *  - Ok(Some(CSSToken::Delim)) =>
 *
 */
use super::super::check::{is_identifier, is_name_code_point, is_valid_escape};
use super::super::consume;
use super::super::error::ParseError;
use super::super::tokens::{CSSToken, HashFlag};
use super::ParseResult;
use std::char;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

const NUMBER_START: char = '#';

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    let next = points.peek();

    if next.is_none() {
        return Ok(None);
    };

    let ch = next.unwrap();

    // Check if it's a # if not is doesn't apply
    if ch.partial_cmp(&NUMBER_START) == Some(Ordering::Equal) {
        // Consume the #
        *position += 1;
        points.next();

        // Check the next character
        if let Some(ch) = points.peek() {
            // If the point is a name code point or a valid escape then it's being used as an ID
            if is_name_code_point(*ch) || is_valid_escape(points) {
                // If the next 3 characters would start an identifier
                let identifier = is_identifier(points);

                // Consume the name
                let result = consume::name(points, position);

                match result {
                    Err(e) => return Err(e),
                    Ok(name) => {
                        return Ok(Some(CSSToken::Hash(
                            name,
                            if identifier {
                                HashFlag::Id
                            } else {
                                HashFlag::Unrestricted
                            },
                        )));
                    }
                };

            // Otherwise the # is being used a delimiter
            } else {
                *position += 1;
                let ch = points.next().unwrap();

                return Ok(Some(CSSToken::Delim(ch)));
            }
        } else {
            return Err(ParseError {
                token: None,
                at: *position,
                error_text: "Unexpected EOF when creating hash",
            });
        }
    }

    // If it doesn't match then this parser doesn't match
    Ok(None)
}
