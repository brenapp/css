/**
 * Parses CSS Comments
 *
 * Returns a Result:
 *  Ok(Some) => If it matched a comment
 *  Ok(None) => If it matched nothing
 *  Err(ParseError) => If it matches
 *
 */
use super::super::error::ParseError;
use super::super::tokens::CSSToken;
use super::lookahead;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub const COMMENT_START: [char; 2] = ['/', '*'];
pub const COMMENT_END: [char; 2] = ['*', '/'];

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    // Look ahead to see if the next charaters match a character
    let comment_starts = lookahead(points, &COMMENT_START);

    if comment_starts {
        // Lookahead to the end of the comment
        let mut empty = false;

        while !empty {
            let end = lookahead(points, &COMMENT_END);
            if end {
                // Advance the pointer to consume the closing brackets
                points.next();
                points.next();

                *position += 2;

                return Ok(Some(CSSToken::Comment));
            }

            *position += 1;
            empty = points.next().is_none();
        }

        // If we've reach the end of the iterator return a parse error
        Err(ParseError {
            token: Some(CSSToken::Comment),
            error_text: "Unexpected End Of File (EOF)",
            at: *position,
        })
    } else {
        Ok(None)
    }
}
