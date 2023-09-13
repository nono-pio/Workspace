use crate::api::grammar::fragment::Fragment;
use crate::api::grammar::rules::{Context, Rule, RuleStatus};
use crate::api::grammar::token::{TokenDefinition, Tokenizer};

pub struct Grammar {
    tokens_definition: Vec<TokenDefinition>,
    rules: Vec<Box<dyn Rule>>,
    fragments: Vec<Fragment>,
    main_fragment_index: usize,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            tokens_definition: Vec::new(),
            rules: Vec::new(),
            fragments: Vec::new(),
            main_fragment_index: 0,
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

    pub fn add_token_definition(mut self, token_definition: TokenDefinition) -> Self {
        self.tokens_definition.push(token_definition);
        self
    }

    pub fn add_rule(mut self, rule: Box<dyn Rule>) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn add_fragment(mut self, fragment: Fragment) -> Self {
        self.fragments.push(fragment);
        self
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
        self.rules[self.fragments[index].get_rule_index()].as_ref()
    }
}
