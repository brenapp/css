/**
 * Tests for certain classes of code points or certain algorithms to check if a stream of code points meet a criteria
 */

use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn equal(a: &char, b: &char) -> bool {
    a.partial_cmp(b) == Some(Ordering::Equal)
}

pub fn next_char_equals(iter: &mut Peekable<Chars>, ch: &char) -> bool {
    let next = iter.peek();

    next.is_some() && equal(next.unwrap(), ch)
}

 pub fn is_name_start_code_point(point: char) -> bool {
    point.is_ascii_alphabetic() || // letter
    point as u32 > 0x0080 || // non-ASCII code point
    point == '_' // U+005F LOW LINE (_)
}

pub fn is_name_code_point(point: char) -> bool {
    // Name start code point
    is_name_start_code_point(point) ||

    // Digit
    point.is_ascii_digit() || 

    // U+002D HYPHEN-MINUS (-)
    point == '-'
}


pub const MAX_CODE_POINT: u32 = 0x10FFFF;

pub fn is_surrogate(num: u32) -> bool {
    0xD800 <= num && num <= 0xDFFF
}

pub fn is_nonprintable(ch: &char) -> bool {
    let code = *ch as u32;

    // A code point between U+0000 NULL and U+0008 BACKSPACE inclusive
    code <= 0x008 ||

    // U+000B LINE TABULATION
    code == 0x000B ||

    // code point between U+000E SHIFT OUT and U+001F INFORMATION SEPARATOR ONE
    code >= 0x000E && code <= 0x001F ||

    // or U+007F DELETE
    code == 0x007F


}


pub fn is_whitespace(ch: &char) -> bool {
    // U+000A LINE FEED
    ch.partial_cmp(&'\n') == Some(Ordering::Equal) || 
    
    // U+0009 CHARACTER TABULATION
    ch.partial_cmp(&' ') == Some(Ordering::Equal) ||

    // U+0020 SPACE
    ch.partial_cmp(&' ') == Some(Ordering::Equal)
}


// 4.3.8. Check if two code points are a valid escape
pub fn is_valid_escape(points: &mut Peekable<Chars>) -> bool {
    
    if let Some(ch) = points.peek() {

        if equal(ch, &'\\') {

            // Clone to prevent consuming points in the main stream
            let mut points = points.clone();

            // Consume the \
            points.next();

            // Now, look at the other code point
            if let Some(ch) = points.peek() {

                // If it's a not newline it's a valid escape
                !equal(ch, &'\n')
    
            } else {
                false
            }

        } else {
            false
        }


    } else {
        false
    }


}

//4.3.9. Check if three code points would start an identifier (Look ahead, consumes nothing)
pub fn is_identifier(points: &mut Peekable<Chars>) -> bool {

    // Because this algorithm is not allowed to consume any additional code points, we need to clone the iterator here
    // However, if the first point is not a U+002D HYPHEN-MINUS a name start code point, a U+005C REVERSE SOLIDUS (\)
    // then we can return false immediately without cloning

    if let Some(ch) = points.peek() {

        // Valid identifier starts

        // U+002D HYPHEN-MINUS
        if equal(ch, &'-') {

            // Now we have to clone the points to avoid consuming anything from the main stream
            let mut points = points.clone();

            // ch here
           if let Some(ch) = points.next() {

                if is_name_start_code_point(ch) || ch == '-' {
                    true
                } else if is_valid_escape(&mut points) {
                    true
                } else {
                    false
                }

           } else {
               false
           }



        } else if is_name_start_code_point(*ch) {
            true
        } else if equal(ch, &'\\') {
            is_valid_escape(points)
        } else {
            false
        }


    } else {
        false
    }
}

// 4.3.10. Check if three code points would start a number
pub fn is_number(points: &mut Peekable<Chars>) -> bool {

    // Lets look at the first digit to determine if we need to clone
    if let Some(ch) = points.peek() {

        // U+002B PLUS SIGN (+)
        // U+002D HYPHEN-MINUS (-)
        if equal(ch, &'+') || equal(ch, &'+') { 

            // Clone points and advance to check the second
            let mut points = points.clone();
            points.next();


            if let Some(second) = points.next() {

                // If the second character is a digit, then we're good (like +4)
                if second.is_ascii_digit() {
                    true

                // If it's a digit, then it's a number if the third char is a digit (like -.4) 
                } else if second == '.' {

                    if let Some(third) = points.next() {
                        third.is_ascii_digit()
                    } else {
                        false
                    }

                } else {
                    false
                }

            // Second char EOF    
            } else {
                false
            }

        // U+002E FULL STOP (.)
        } else if equal(ch, &'.') {

            // If it starts with a . the second char needs to be a digit (like .4)
            if let Some(second) = points.next() {
                second.is_ascii_digit()
            } else {
                false
            }

        // Now it's only a number if it starts with a digit (like 4)
        } else {
            ch.is_ascii_digit()
        }

    // First char EOF
    } else {
        false
    }

}

