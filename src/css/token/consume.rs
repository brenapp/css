/**
 * Utilites to consume various aspects
 */
use std::iter::Peekable;
use std::str::Chars;

use super::check::{
    equal, is_identifier, is_name_code_point, is_nonprintable, is_surrogate, is_valid_escape,
    is_whitespace, next_char_equals, MAX_CODE_POINT,
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
        for _ in 0..5 {
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
fn digits(points: &mut Peekable<Chars>, position: &mut i32) -> String {
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
    let mut iter = string.chars().peekable();

    // Sign
    let mut s = 1.0;
    if let Some(sign) = iter.peek() {
        if equal(sign, &'-') {
            // Advance the pointer and set sign
            s = -1.0;
            iter.next();
        } else if equal(sign, &'+') {
            iter.next();
        }
    };

    // Digits
    let num = digits(&mut iter, &mut 0);
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
    let num = digits(&mut iter, &mut d);
    let f: f64 = if num.len() < 1 {
        0.0
    } else {
        match num.parse::<f64>() {
            Ok(parsed) => parsed,
            Err(e) => return Err(e),
        }
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
    let num = digits(&mut iter, &mut d);
    let e: f64 = if num.len() < 1 {
        0.0
    } else {
        match num.parse::<f64>() {
            Ok(parsed) => parsed,
            Err(e) => return Err(e),
        }
    };

    // Create the number from the segments
    Ok(s * (i + f * 10f64.powf(-d as f64)) * 10f64.powf(t * e))
}

// 4.3.12. Consume a number
pub fn number(
    points: &mut Peekable<Chars>,
    position: &mut i32,
) -> Result<(f64, NumericFlag), ParseError> {
    let mut flag = NumericFlag::Integer;
    let mut repr = String::new();

    if let Some(ch) = points.peek() {
        // If there is a sign preceeding the number, add it to the string
        if equal(ch, &'+') || equal(ch, &'-') {
            *position += 1;
            let ch = points.next().unwrap();
            repr.push(ch);
        }

        // Consume digits if any and push them onto repr
        let string = digits(points, position);
        repr.push_str(string.as_str());

        // Check for decimals
        if next_char_equals(points, &'.') {
            let mut lookahead = points.clone();
            lookahead.next();

            // If we have digits after the decimal place
            if let Some(ch) = lookahead.next() {
                if ch.is_ascii_digit() {
                    // Consume the .
                    *position += 1;
                    repr.push(points.next().unwrap());

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
                    // Consume the e
                    let e = points.next().unwrap();
                    repr.push(e);

                    let string = digits(points, position);
                    repr.push_str(string.as_str());
                }
            }
        }

        // Convert the string to number and return with type
        let number = to_number(repr);

        match number {
            Ok(num) => Ok((num, flag)),
            Err(_) => Err(ParseError {
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
                // Consume the %
                *position += 1;
                points.next();

                Ok(CSSToken::Percentage(number))
            // Otherwise it's a number
            } else {
                Ok(CSSToken::Number(number, flag))
            }
        }
    }
}

// "Consume as much whitespace as possible"
pub fn whitespace(points: &mut Peekable<Chars>, position: &mut i32) {
    let mut peek = points.peek();

    // Consume as much whitespace as possible
    while peek.is_some() && is_whitespace(peek.unwrap()) {
        *position += 1;
        points.next();
        peek = points.peek();
    }
}

// 4.3.14. Consume the remnants of a bad url
pub fn bad_url_remnant(points: &mut Peekable<Chars>, position: &mut i32) {
    loop {
        *position += 1;
        let next = points.next();

        loop {
            match next {
                None => break,
                Some(ch) => {
                    if ch == ')' {
                        break;
                    } else if is_valid_escape(points) {
                        let _ = escape(points, position);
                    }
                }
            };
        }
    }
}

pub fn url_token(points: &mut Peekable<Chars>, position: &mut i32) -> Result<CSSToken, ParseError> {
    let mut string = String::new();

    // Consume as much whitespace as possible
    whitespace(points, position);

    // Repeatedly consume the next input code point from the stream
    loop {
        *position += 1;
        let next = points.next();

        match next {
            // EOF is a parse error
            None => {
                return Err(ParseError {
                    token: Some(CSSToken::URL(string)),
                    at: *position,
                    error_text: "Unexpected End of File (EOF)",
                })
            }

            Some(ch) => {
                // If the character is ), close the URL
                if ch == ')' {
                    return Ok(CSSToken::URL(string));

                // If it's whitespace, consume as much whitespace as possible
                } else if is_whitespace(&ch) {
                    whitespace(points, position);

                // These characters indicate a bad URL
                } else if ch == '"' || ch == '\'' || ch == '(' || is_nonprintable(&ch) {
                    // Consume the remnants of the bad url and return a token
                    bad_url_remnant(points, position);

                    return Err(ParseError {
                        error_text: "Bad URL detected",
                        at: *position,
                        token: Some(CSSToken::BadURL),
                    });

                // Check and account for valid escapes
                } else if ch == '\\' {
                    if is_valid_escape(points) {
                        match escape(points, position) {
                            Ok(ch) => string.push(ch),
                            Err(e) => return Err(e),
                        }

                    // If it's not a valid escape, this is a Bad URL
                    } else {
                        bad_url_remnant(points, position);

                        return Err(ParseError {
                            error_text: "Bad URL detected",
                            at: *position,
                            token: Some(CSSToken::BadURL),
                        });
                    }
                } else {
                    string.push(ch);
                }
            }
        }
    }
}

// 4.3.4. Consume an ident-like token
pub fn ident_like_token(
    points: &mut Peekable<Chars>,
    position: &mut i32,
) -> Result<CSSToken, ParseError> {
    let string = match name(points, position) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    // If string’s value is an ASCII case-insensitive match for "url", and the next input code point is U+0028 LEFT PARENTHESIS ((), consume it.
    if string.to_ascii_lowercase() == String::from("url") && next_char_equals(points, &'(') {
        *position += 1;
        points.next();

        // While the next two characters are whitespace, consume characters
        loop {
            // @TODO OPTIMIZE THIS
            // CLONING THE ITERATOR ON EVERY LOOP IS AWFUL
            // BUT ITERTOOLS MULTIPEEK FUCKS UP EVERYTHING
            let mut lookahead = points.clone();
            let first = lookahead.next();

            // If the first char is not whitespace
            if first.is_none() || !is_whitespace(&first.unwrap()) {
                break;
            }

            let second = lookahead.next();

            // If the second char is not whitespace
            if second.is_none() || !is_whitespace(&second.unwrap()) {
                break;
            }

            // Consume the code point
            points.next();
        }

        // If the next one or two input code points are
        // U+0022 QUOTATION MARK ("),
        // U+0027 APOSTROPHE ('), or
        // whitespace followed by U+0022 QUOTATION MARK (") or U+0027 APOSTROPHE ('),
        // then create a <function-token> with its value set to string and return it.

        // If the next char is whitespace
        let peek = points.peek();

        // Check if the next fchar is " or '
        if peek.is_some() && (equal(peek.unwrap(), &'"') || equal(peek.unwrap(), &'\'')) {
            return Ok(CSSToken::Function(string));
        } else if peek.is_some() && is_whitespace(peek.unwrap()) {
            let mut points = points.clone();
            let next = points.next();

            // Then we gotta check if the next character after that is ' or "
            match next {
                Some(c) => {
                    if c == '\'' || c == '"' {
                        return Ok(CSSToken::Function(string));
                    }
                }
                None => {}
            };
        // Otherwise, consume a url token, and return it.
        } else {
            return url_token(points, position);
        }
    }

    // Otherwise, if the next input code point is U+0028 LEFT PARENTHESIS ((), consume it
    if next_char_equals(points, &'(') {
        *position += 1;
        points.next();

        return Ok(CSSToken::Function(string));
    }

    Ok(CSSToken::Ident(string))
}
