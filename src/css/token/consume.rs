/**
 * Utilites to consume various aspects
 */
use std::iter::Peekable;
use std::str::Chars;

use super::check::{
    equal, is_identifier, is_name_code_point, is_surrogate, is_valid_escape, next_char_equals,
    MAX_CODE_POINT,
};
use super::error::ParseError;
use super::tokens::{CSSToken, NumericFlag};
use std::char;
use std::i32;
use std::num::ParseFloatError;

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

// Consumes digits (convience method)
fn digits(points: &Peekable<Chars>, position: &mut i32) -> String {
    let mut peek = points.peek();
    let mut repr = String::new();

    while peek.is_some() && peek.unwrap().is_ascii_digit() {
        // Consume the digit
        *position += 1;
        let digit = points.next().unwrap();

        repr.push(digit);

        peek = points.peek();
    }

    repr
}

// 4.3.13. Convert a string to a number
pub fn to_number(string: String) -> Result<f64, ParseFloatError> {
    let iter = string.chars().peekable();

    // Sign
    let mut s = 1.0;
    if let Some(sign) = iter.peek() {
        if equal(sign, &'-') {
            // Advance the pointer and set sign
            s = -1.0;
            iter.next();
        }
    };

    // Digits
    let num = digits(&iter, &mut 0);
    let i: f64 = match num.parse::<f64>() {
        Ok(parsed) => parsed,
        Err(e) => return Err(e),
    };

    // Decimal point
    if let Some(sign) = iter.peek() {
        if equal(sign, &'.') {
            // Advance the pointer through the decimal points
            iter.next();
        }
    };

    // Fraction digits
    let mut d = 0;
    let num = digits(&iter, &mut d);
    let f: f64 = match num.parse::<f64>() {
        Ok(parsed) => parsed,
        Err(e) => return Err(e),
    };

    // Exponential Indicator
    if let Some(sign) = iter.peek() {
        if equal(sign, &'e') || equal(sign, &'E') {
            // Advance the pointer through the decimal points
            iter.next();
        }
    };

    // Expoential Sign
    let mut t = 1.0;
    if let Some(sign) = iter.peek() {
        if equal(sign, &'-') {
            // Advance the pointer and set sign
            t = -1.0;
            iter.next();
        }
    };

    // Exponent digits
    let mut e = 0;
    let num = digits(&iter, &mut d);
    let e: f64 = match num.parse::<f64>() {
        Ok(parsed) => parsed,
        Err(e) => return Err(e),
    };

    // Create the number from the segments
    Ok(s * (i + f * 10f64.powf(-d as f64)) * 10f64.powf(t * e))
}

// 4.3.12. Consume a number
pub fn number(
    points: &mut Peekable<Chars>,
    position: &mut i32,
) -> Result<(f64, NumericFlag), ParseError> {
    let flag = NumericFlag::Integer;
    let mut repr = String::new();

    if let Some(ch) = points.next() {
        // If there is a sign preceeding the number, add it to the string
        if ch == '+' || ch == '-' {
            repr.push(ch);

            // Advance the pointer beyond the + -
            *position += 1;
            points.next();
        }

        // Consume digits if any and push them onto repr
        let string = digits(points, position);
        repr.push_str(string.as_str());

        let peek = points.peek();

        // Check for decimals
        if peek.is_some() && equal(peek.unwrap(), &'.') {
            let mut lookahead = points.clone();
            lookahead.next();

            // If we have digits after the decimal place
            if let Some(char) = lookahead.next() {
                if char.is_ascii_digit() {
                    // Consume digits if any and push them onto repr
                    let string = digits(points, position);
                    repr.push_str(string.as_str());

                    // Set type to number
                    flag = NumericFlag::Number;
                }
            }
        }

        // Check for expoentials
        let peek = points.peek();

        // If the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E) or U+0065 LATIN SMALL LETTER E (e)
        if peek.is_some() && (equal(peek.unwrap(), &'e') || equal(peek.unwrap(), &'E')) {
            let mut lookahead = points.clone();
            lookahead.next();

            // optionally followed by U+002D HYPHEN-MINUS (-) or U+002B PLUS SIGN (+)
            if let Some(second) = lookahead.next() {
                // If it's a digit or + or -, then the third has to be a digit
                if second == '+' || second == '-' {
                    if let Some(third) = lookahead.next() {
                        if third.is_ascii_digit() {
                            // Consume both (like e+) and add the to repr
                            *position += 2;
                            let e = points.next().unwrap();
                            let pm = points.next().unwrap();

                            repr.push(e);
                            repr.push(pm);

                            // Consume digits
                            // Consume digits if any and push them onto repr
                            let string = digits(points, position);
                            repr.push_str(string.as_str());
                        }
                    }

                // Otherwise, if the second is the ASCII digit, then consume digits to add to repr
                } else if second.is_ascii_digit() {
                    let string = digits(points, position);
                    repr.push_str(string.as_str());
                }
            }
        }

        // Convert the string to number and return with type
        let number = to_number(repr);

        match number {
            Ok(num) => Ok((num, flag)),
            Err(e) => Err(ParseError {
                token: None,
                at: *position,
                error_text: "Could not parse number",
            }),
        }
    } else {
        Err(ParseError {
            token: None,
            at: *position,
            error_text: "Unexpected End Of File (EOF) parsing number",
        })
    }
}

// 4.3.3. Consume a numeric token
pub fn numeric_token(
    points: &mut Peekable<Chars>,
    position: &mut i32,
) -> Result<CSSToken, ParseError> {
    match number(points, position) {
        Err(e) => return Err(e),
        Ok((number, flag)) => {
            // If next 3 code points are an identifier, then this is a dimension
            if is_identifier(points) {
                let result = name(points, position);

                match result {
                    Ok(dim) => Ok(CSSToken::Dimension(number, NumericFlag::Number, dim)),
                    Err(e) => Err(e),
                }
            // If the next char is a % then it's a percentage
            } else if next_char_equals(points, &'%') {
                Ok(CSSToken::Percentage(number))
            // Otherwise it's a number
            } else {
                Ok(CSSToken::Number(number, flag))
            }
        }
    }
}
