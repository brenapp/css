pub mod error;
pub mod parsers;
pub mod tokens;

/**
 * CSS Tokenizer
 * https://www.w3.org/TR/css-syntax-3/#tokenizing-and-parsing
 */

pub fn tokenize(contents: String) -> Result<Vec<tokens::CSSToken>, error::ParseError> {
    let mut tokens: Vec<tokens::CSSToken> = Vec::new();
    let mut iter = contents.chars();

    let mut done = false;

    while !done {
        let result = parsers::parse(&mut iter);

        // Go through the possible options
        match result {
            // If the parser returned an error propagate that up
            Err(e) => return Err(e),

            // Next, if it returned a token then push it
            // Else if no token was returned we're at the EOF
            Ok(result) => match result {
                Some(token) => tokens.push(token),
                None => done = true,
            },
        }
    }

    println!("{:?}", tokens);

    Ok(tokens)
}
