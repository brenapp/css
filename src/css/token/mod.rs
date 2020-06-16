pub mod error;
pub mod parsers;
pub mod tokens;

use tokens::CSSToken;

/**
 * CSS Tokenizer
 * https://www.w3.org/TR/css-syntax-3/#tokenizing-and-parsing
 */

pub fn preprocess(contents: String) -> String {
    contents
}

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
