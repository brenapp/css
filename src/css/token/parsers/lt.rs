/**
 * Less than matching
 *
 */
use super::super::check::next_char_equals;
use super::super::error::ParseError;
use super::super::tokens::CSSToken;
use super::lookahead;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

const CDO: [char; 3] = ['!', '-', '-'];

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if next_char_equals(points, &'<') {
        *position += 1;
        points.next();

        // Check for CDO
        if lookahead(points, &CDO) {
            // Consume CDO
            *position += 3;
            points.next();
            points.next();
            points.next();

            Ok(Some(CSSToken::CDO))
        } else {
            *position += 1;
            let next = points.next();

            match next {
                Some(c) => Ok(Some(CSSToken::Delim(c))),
                None => Err(ParseError {
                    at: *position,
                    token: None,
                    error_text: "Unexpected End of File (EoF) around <",
                }),
            }
        }
    } else {
        Ok(None)
    }
}
