/**
 * Tests for certain classes of code points or certain algorithms to check if a stream of code points meet a criteria
 */

use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

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

// 4.3.8. Check if two code points are a valid escape
pub fn is_valid_escape(points: &mut Peekable<Chars>) -> bool {
    
    if let Some(ch) = points.peek() {

        if ch.partial_cmp(&'\\') == Some(Ordering::Equal) {

            // Clone to prevent consuming points in the main stream
            let mut points = points.clone();

            // Consume the \
            points.next();

            // Now, look at the other code point
            if let Some(ch) = points.peek() {

                // If it's a not newline it's a valid escape
                ch.partial_cmp(&'\n') != Some(Ordering::Equal)
    
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
        if ch.partial_cmp(&'-') == Some(Ordering::Equal) {

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
        } else if ch.partial_cmp(&'\\') == Some(Ordering::Equal) {
            is_valid_escape(points)
        } else {
            false
        }


    } else {
        false
    }
}

pub const MAX_CODE_POINT: u32 = 0x10FFFF;

pub fn is_surrogate(num: u32) -> bool {
    0xD800 <= num && num <= 0xDFFF
}