use regex::Regex;

// region: ---Token

#[derive(Debug)]
pub struct Token<'a> {
    definition_index: usize,
    slice: &'a str,
    position: (usize, usize),
}

impl<'a> Token<'a> {
    pub fn new(definition_index: usize, slice: &str, position: (usize, usize)) -> Token {
        Token {
            definition_index,
            slice,
            position,
        }
    }

    pub fn definition_index(&self) -> usize {
        self.definition_index
    }

    pub fn slice(&self) -> &str {
        self.slice
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn equal(&self, definition_index: usize) -> bool {
        self.definition_index == definition_index
    }
}

// endregion

// region: ---Token Definition

#[derive(Debug)]
pub enum Pattern {
    Keyword(Box<str>),
    Regex(Regex),
}

#[derive(Debug)]
pub struct TokenDefinition {
    name: Box<str>,
    pattern: Pattern,
}

impl TokenDefinition {
    pub fn new_regex(name: &str, regex: &str) -> Self {
        TokenDefinition {
            name: Box::from(name),
            pattern: Pattern::Regex(Regex::new(regex).unwrap()),
        }
    }

    pub fn new_keyword(name: &str, keyword: &str) -> Self {
        TokenDefinition {
            name: Box::from(name),
            pattern: Pattern::Keyword(Box::from(keyword)),
        }
    }
    
    pub fn new(name: &str, pattern: Pattern) -> Self {
        TokenDefinition {
            name: Box::from(name),
            pattern,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

// endregion

// region: ---Tokenizer

#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a str,                               // text to tokenize
    pub(crate) current_text: &'a str,            // slice of text that need to be tokenize
    tokens_definition: &'a Vec<TokenDefinition>, // tokens definition
    tokens: Vec<Token<'a>>,                      // tokens
    current_position: (usize, usize),            //position
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str, tokens_definition: &'a Vec<TokenDefinition>) -> Tokenizer<'a> {
        Tokenizer {
            text,
            current_text: text,
            tokens_definition,
            tokens: Vec::new(),
            current_position: (0, 0),
        }
    }

    pub fn get_token(&mut self, index: usize) -> Option<&Token<'a>> {
        if index >= self.tokens.len() {
            self.generate_token()
        } else {
            Some(&self.tokens[index])
        }
    }

    // return the token corresponding to the definition if it is already generated or generate it
    pub fn get_token_from_token_definition(
        &mut self,
        index: usize,
        definition_index: usize,
    ) -> Option<&Token<'a>> {
        if index >= self.tokens.len() {
            self.generate_token_from_token_definition(definition_index)
        } else {
            let token = &self.tokens[index];
            if token.definition_index() == definition_index {
                Some(token)
            } else {
                None
            }
        }
    }

    pub fn definition_len(&self) -> usize {
        self.tokens_definition.len()
    }

    pub fn generate_token(&mut self) -> Option<&Token<'a>> {
        // check if there is text to tokenize
        if self.current_text.is_empty() {
            return None;
        }

        // try to match the text with a token definition
        for (definition_index, definition) in self.tokens_definition.iter().enumerate() {
            match &definition.pattern {
                Pattern::Keyword(str) => {
                    if self.current_text.starts_with(str.as_ref()) {
                        // make the token
                        let token = Token::new(definition_index, str, self.current_position);
                        self.tokens.push(token);

                        // update state
                        self.current_position.1 += str.len();

                        // return the token
                        return Some(self.tokens.last().unwrap());
                    }
                }
                Pattern::Regex(regex) => {
                    // try to match the text with the regex
                    if let Some(match_) = regex.find(self.current_text) {
                        // match string
                        let match_str = match_.as_str();

                        // make the token
                        let token = Token::new(definition_index, match_str, self.current_position);
                        self.tokens.push(token);

                        // update state
                        let last_index = match_.range().end;
                        self.current_text = &self.current_text[last_index..];
                        self.current_position.1 += last_index;

                        // return the token
                        return Some(self.tokens.last().unwrap());
                    }
                }
            }
        }

        // no token found
        None
    }

    pub fn generate_token_from_token_definition(
        &mut self,
        definition_index: usize,
    ) -> Option<&Token<'a>> {
        // check if there is text to tokenize
        match &self.tokens_definition[definition_index].pattern {
            Pattern::Keyword(str) => {
                if self.current_text.starts_with(str.as_ref()) {
                    // make the token
                    let token = Token::new(definition_index, str, self.current_position);
                    self.tokens.push(token);

                    // update state
                    let len = str.len();
                    self.current_position.1 += len;
                    self.current_text = &self.current_text[len..];

                    // return the token
                    return Some(self.tokens.last().unwrap());
                }
            }
            Pattern::Regex(regex) => {
                // try to match the text with the regex
                if let Some(match_) = regex.find(self.current_text) {
                    // match string
                    let match_str = match_.as_str();

                    // make the token
                    let token = Token::new(definition_index, match_str, self.current_position);
                    self.tokens.push(token);

                    // update state
                    let last_index = match_.range().end;
                    self.current_text = &self.current_text[last_index..];
                    self.current_position.1 = last_index;

                    // return the token
                    return Some(self.tokens.last().unwrap());
                }
            }
        }

        // no token found
        None
    }

    pub fn get_all_tokens(&mut self) -> &Vec<Token<'a>> {
        while self.generate_token().is_some() {}
        &self.tokens
    }
}

// endregion
