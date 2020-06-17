/**
 * Utilites to consume various aspects
 */
use std::iter::Peekable;
use std::str::Chars;

use super::check::{is_name_code_point, is_surrogate, is_valid_escape, MAX_CODE_POINT};
use super::error::ParseError;
use super::tokens::CSSToken;
use std::char;

// 4.3.11. Consume a name
pub fn name(points: &mut Peekable<Chars>, position: &mut i32) -> Result<String, ParseError> {
    let mut name = String::new();

    // Repeatedly consume code points
    loop {
        let next = points.peek();

        match next {
            Some(ch) => {
                if is_name_code_point(*ch) {
                    *position += 1;
                    let ch = points.next().unwrap();
                    name.push(ch);
                } else if is_valid_escape(points) {
                    let result = escape(points, position);

                    match result {
                        Ok(ch) => name.push(ch),
                        Err(e) => return Err(e),
                    };
                } else {
                    return Ok(name);
                }
            }
            None => return Ok(name),
        };
    }
}

// 4.3.7. Consume an escaped code point
pub fn escape(points: &mut Peekable<Chars>, position: &mut i32) -> Result<char, ParseError> {
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

    // If we're specifying a hex here
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
        let ch = match u32::from_str_radix(hex.as_str(), 16) {
            Ok(number) => {
                // If the number is zero, represents a surrogate, or greater than the max allowed code point then return U+FFFD REPLACEMENT CHARACTER (�)
                if number == 0 || is_surrogate(number) || number > MAX_CODE_POINT {
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
            Err(_) => {
                return Err(ParseError {
                    token: Some(CSSToken::BadString),
                    at: *position,
                    error_text: "Encountered problem parsing escape hex sequence",
                })
            }
        };

        // Push the resolved character from the escape
        Ok(ch)

    // If not a hex digit, then just return the actual character as the escape
    } else {
        Ok(ch)
    }
}
