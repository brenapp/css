/**
 * Matches ( and ) tokens
 */
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if let Some(ch) = points.peek() {
        // Match for left paren
        if ch.partial_cmp(&'(') == Some(Ordering::Equal) {
            // Consume the token
            *position += 1;
            points.next();

            Ok(Some(CSSToken::LeftParentheses))

        // Match for right paren
        } else if ch.partial_cmp(&')') == Some(Ordering::Equal) {
            // Consume the token
            *position += 1;
            points.next();

            Ok(Some(CSSToken::RightParentheses))

        // If it doesn't match either of these return
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
