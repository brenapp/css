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
use std::str::Chars;

pub const COMMENT_START: [char; 2] = ['/', '*'];
pub const COMMENT_END: [char; 2] = ['*', '/'];

pub fn parse(points: &mut Chars) -> ParseResult {
    // Look ahead to see if the next charaters match a character
    let comment_starts = lookahead(points, &COMMENT_START);

    println!("Comment parse {:?}", points);

    if comment_starts {
        // Lookahead to the end of the comment

        let mut empty = false;

        while !empty {
            let end = lookahead(points, &COMMENT_END);
            if end {
                return Ok(Some(CSSToken::Comment));
            }

            empty = points.next().is_none();
        }

        // If we've reach the end of the iterator return a parse error
        Err(ParseError {
            token: CSSToken::Comment,
            error_text: "Unexpected End Of File (EOF) when parsing comment body",
            at: 0,
        })
    } else {
        Ok(None)
    }
}

// for _ in points {
//     let end = lookahead(points, &COMMENT_END);

//     // If we found the end return
//     if end {
//         return Ok(Some(CSSToken::Comment));
//     }
// }

// // If we reached the end and there was no end comment, it's a parse error
// Err(ParseError {
//     token: CSSToken::Comment,
//     error_text: "Unexpected EOF when parsing comment",
//     at: 0,
// })
