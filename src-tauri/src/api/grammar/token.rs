use std::rc::Rc;

use serde::{Deserialize, Serialize};

// ( line, col )
type TextPosition = (u32, u32);

// region: ---Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    regex: String,
    name: String,
}

impl Token {
    pub fn new(name: &str, regex: &str) -> Token {
        Token {
            regex: regex.to_string(),
            name: name.to_string(),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_regex(&self) -> &str {
        self.regex.as_str()
    }
}

// endregion

// region: ---TokenInstance
struct TokenInstance {
    value: String,
    token: Rc<Token>,
    start_pos: TextPosition,
}

impl TokenInstance {
    fn new(token: Rc<Token>, value: &str, start_pos: TextPosition) -> TokenInstance {
        TokenInstance {
            value: value.to_string(),
            token,
            start_pos,
        }
    }
}
// endregion
