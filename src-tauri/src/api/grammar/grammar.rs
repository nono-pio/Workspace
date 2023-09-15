use crate::api::grammar::fragment::Fragment;
use crate::api::grammar::rules::{Context, Rule, RuleStatus};
use crate::api::grammar::token::{TokenDefinition, Tokenizer};

#[derive(Debug)]
pub struct Grammar {
    name: Box<str>,
    tokens_definition: Vec<TokenDefinition>,
    fragments: Vec<Fragment>,
    main_fragment_index: usize,
}

impl Grammar {
    pub fn new(
        name: &str,
        tokens_definition: Vec<TokenDefinition>,
        fragments: Vec<Fragment>,
        main_fragment: usize,
    ) -> Self {
        Grammar {
            name: Box::from(name),
            tokens_definition,
            fragments,
            main_fragment_index: main_fragment,
        }
    }

    pub fn parse(&self, text: &str) -> Option<Context> {
        let mut tokenizer = Tokenizer::new(text, &self.tokens_definition);
        match self
            .get_rule_of_fragment(self.main_fragment_index)
            .parse(&mut tokenizer, self, 0)
        {
            RuleStatus::Valid(context, _) => Some(context),
            RuleStatus::Invalid => None,
        }
    }

    pub fn get_tokens_definition(&self) -> &Vec<TokenDefinition> {
        &self.tokens_definition
    }

    pub fn get_token_definition(&self, index: usize) -> &TokenDefinition {
        &self.tokens_definition[index]
    }

    pub fn get_fragment(&self, index: usize) -> &Fragment {
        &self.fragments[index]
    }

    pub fn get_rule_of_fragment(&self, index: usize) -> &dyn Rule {
        self.fragments[index].get_rule()
    }
}
