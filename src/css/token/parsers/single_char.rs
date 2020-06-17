/**
 * Matches context free tokens: parentheses, comma, etc
 */
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(
    points: &mut Peekable<Chars>,
    position: &mut i32,
    cmp: char,
    token: CSSToken,
) -> ParseResult {
    if let Some(ch) = points.peek() {
        // Match for char
        if ch.partial_cmp(&cmp) == Some(Ordering::Equal) {
            // Consume the token
            *position += 1;
            points.next();

            Ok(Some(token))

        // Match for right paren
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
