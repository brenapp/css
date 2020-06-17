use super::super::consume;
/**
 * Parses CSS Strings
 *
 * Returns a Result:
 *  Ok(String(text)) => Found a String
 *  Ok(None) => No String
 *  ParseError => BadString
 */
use super::super::error::ParseError;
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::char;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(
    points: &mut Peekable<Chars>,
    position: &mut i32,
    string_delimit: char,
) -> ParseResult {
    let next = points.peek();

    match next {
        // Match for string starts
        Some(ch) if ch.partial_cmp(&string_delimit) == Some(Ordering::Equal) => {
            // Consume the " (we just peeked when comparing)
            points.next();
            *position += 1;

            // Now we are consuming a string token
            let mut token = String::new();

            // Build the string token
            loop {
                let next = points.peek();

                // EOF (parse error)
                if next.is_none() {
                    return Err(ParseError {
                        token: Some(CSSToken::String(token)),
                        at: *position,
                        error_text: "Unexpected End Of File (EOF)",
                    });
                }

                // Now let's look at the character
                let ch = next.unwrap();

                // Ending character
                if ch.partial_cmp(&string_delimit) == Some(Ordering::Equal) {
                    // Consume the code point
                    *position += 1;
                    points.next();

                    return Ok(Some(CSSToken::String(token)));
                }

                // Newline (Bad string Token)
                if ch.partial_cmp(&'\n') == Some(Ordering::Equal) {
                    return Ok(Some(CSSToken::BadString));
                }

                // Escape U+005C REVERSE SOLIDUS (\)
                // TODO: Maybe extract this into it's own file, it's probably applicable in other places
                if ch.partial_cmp(&'\\') == Some(Ordering::Equal) {
                    // Consume the code point
                    *position += 1;
                    points.next();

                    // Look at the next character
                    let next = points.peek();

                    // Do nothing on EOF
                    if next.is_some() {
                        let ch = next.unwrap();

                        // If the next character is a newline, consume it (escaping newlines in strings)
                        if ch.partial_cmp(&'\n') == Some(Ordering::Equal) {
                            *position += 1;
                            points.next();

                        // This is a valid code point escape, consume it
                        } else {
                            let result = consume::escape(points, position);

                            match result {
                                Ok(ch) => token.push(ch),
                                Err(e) => {
                                    return Err(e);
                                }
                            };
                        }
                    }
                } else {
                    // Anything else (append the current code point to the token)
                    *position += 1;
                    let ch = points.next().unwrap();
                    token.push(ch);
                }
            }
        }
        // For non-strings
        _ => Ok(None),
    }
}
