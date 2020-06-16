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
use std::u32;

const STRING_START: char = '"';
const STRING_END: char = '"';

const MAX_CODE_POINT: u32 = 0x10FFFF;

pub fn is_surrogate(num: u32) -> bool {
    0xD800 <= num && num <= 0xDFFF
}

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    let next = points.peek();

    match next {
        // Match for string starts
        Some(ch) if ch.partial_cmp(&STRING_START) == Some(Ordering::Equal) => {
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
                if ch.partial_cmp(&STRING_END) == Some(Ordering::Equal) {
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
                            *position += 1;
                            let next = points.next();

                            // EOF is parse error
                            if next.is_none() {
                                return Err(ParseError {
                                    at: *position,
                                    token: Some(CSSToken::BadString),
                                    error_text: "Unexpected End Of File (EOF)",
                                });
                            }

                            let ch = next.unwrap();

                            // Consume up to 5 hex digits
                            if ch.is_ascii_hexdigit() {
                                let mut hex: String = String::new();

                                hex.push(ch);

                                // Collect the remaining digits
                                for i in 0..5 {
                                    let next = points.peek();

                                    if next.is_none() {
                                        break;
                                    }

                                    let ch = next.unwrap();

                                    // If it's hex, consume it, otherwise break
                                    if ch.is_ascii_hexdigit() {
                                        *position += 1;
                                        let ch = points.next().unwrap();
                                        hex.push(ch);
                                    } else {
                                        break;
                                    }
                                }

                                // Now that we've collected the hex, turn it into a number
                                let ch =
                                    match u32::from_str_radix(hex.as_str(), 16) {
                                        Ok(number) => {
                                            // If the number is zero, represents a surrogate, or greater than the max allowed code point then return U+FFFD REPLACEMENT CHARACTER (�)
                                            if number == 0
                                                || is_surrogate(number)
                                                || number > MAX_CODE_POINT
                                            {
                                                '�'
                                            } else {
                                                let result = char::from_u32(number);

                                                /* If we couldn't parse it return U+FFFD REPLACEMENT CHARACTER (�)
                                                   My assumption is that this code could tecnically use the
                                                   char::from_u32_unchecked() as the spec defines checks in place.
                                                   However, it's probably better to just return � if parsing fails
                                                */
                                                if result.is_none() {
                                                    '�'
                                                } else {
                                                    result.unwrap()
                                                }
                                            }
                                        }

                                        // Parse error (invalid hex)
                                        Err(_) => return Err(ParseError {
                                            token: Some(CSSToken::BadString),
                                            at: *position,
                                            error_text:
                                                "Encountered problem parsing escape hex sequence",
                                        }),
                                    };

                                // Push the resolved character from the escape
                                token.push(ch);

                            // If not a hex digit, then just return the actual character as the escape
                            } else {
                                token.push(ch);
                            }
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
