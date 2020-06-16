pub mod error;
pub mod parsers;
pub mod tokens;

use tokens::CSSToken;

/**
 * CSS Tokenizer
 * https://www.w3.org/TR/css-syntax-3/#tokenizing-and-parsing
 */

/**
 * § 3.3 Stream Preprocessing
 *
 * \r\n => \n
 * \r => \n
 * U+D800 to U+DFFF => U+FFFD REPLACEMENT CHARACTER (�)
 *
 * This was taken from the earlier parts of the Servo project (https://github.com/servo/rust-cssparser/blob/0bad1d50ce955bebbc845b5522f5a99d203a04f3/cssparser.rs#L12) and is highly optimizable
 */
pub fn preprocess(contents: String) -> String {
    contents
        .replace("\r\n", "\n")
        .replace("\r", "\n")
        .replace("\x00", "\u{FFFD}")
}

/**
 * § Tokenization
 *
 * 1. Comments
 * 2. Whitespace
 */
pub fn tokenize(contents: String) -> Result<Vec<tokens::CSSToken>, error::ParseError> {
    // Prepare the contents
    let contents = preprocess(contents);

    let mut tokens: Vec<tokens::CSSToken> = Vec::new();
    let mut iter = contents.chars().peekable();

    let mut done = false;
    let mut position = 0;

    while !done {
        let result = parsers::parse(&mut iter, &mut position);

        // Go through the possible options
        match result {
            // If the parser returned an error propagate that up
            Err(e) => return Err(e),

            // If we're good then push the token onto the token list
            // However if EOF is reached end
            Ok(result) => match result {
                CSSToken::EOF => done = true,
                _ => tokens.push(result),
            },
        }
    }

    Ok(tokens)
}
