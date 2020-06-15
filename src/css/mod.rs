pub mod token;
use std::string::String;

pub fn optimize(contents: String) -> Result<String, token::error::ParseError> {
    let tokens = match token::tokenize(contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e),
    };

    Ok(String::new())
}
