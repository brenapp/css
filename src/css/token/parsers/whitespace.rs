/**
 * Parses CSS Whitespace
 *
 * Returns a Result:
 *  Ok(Some) => If it matched some whitespace
 *  Ok(None) => No whitespace match
 */
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn is_whitespace(ch: &char) -> bool {
    // U+000A LINE FEED
    ch.partial_cmp(&'\n') == Some(Ordering::Equal) || 
    
    // U+0009 CHARACTER TABULATION
    ch.partial_cmp(&' ') == Some(Ordering::Equal) ||

    // U+0020 SPACE
    ch.partial_cmp(&' ') == Some(Ordering::Equal)
}

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    // Check if we're at a whitespace character
    let next = points.peek();

    match next {
        Some(ch) if is_whitespace(&ch) => {

            loop {
                
                let next = points.peek();

                match next {
                    None => { break },
                    Some(ch) => {
                        if !is_whitespace(&ch) {
                            break;
                        }
                    }
                };

                points.next();
                *position+=1;
            };

            Ok(Some(CSSToken::Whitespace))

        }
        _ => Ok(None)
    }
}
