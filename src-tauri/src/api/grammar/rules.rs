use super::{grammar::Grammar, token::Tokenizer};
use std::fmt::{Debug, Formatter};

// region: ---Rule Status
#[derive(Debug)]
pub enum RuleStatus {
    Valid(Context, usize /* Index End Of Context */),
    Invalid,
}
// endregion

// region: ---Context Enum
pub enum Context {
    Token(TokenContext),
    Fragment(FragmentContext),
    Sequence(SequenceContext),
    Or(OrContext),
    Optional(OptionalContext),
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::Token(token_context) => write!(f, "{:?}", token_context),
            Context::Fragment(fragment_context) => write!(f, "{:?}", fragment_context),
            Context::Sequence(sequence_context) => write!(f, "{:?}", sequence_context),
            Context::Or(or_context) => write!(f, "{:?}", or_context),
            Context::Optional(optional_context) => write!(f, "{:?}", optional_context),
        }
    }
}

// endregion

// region: ---Rule Trait
pub trait Rule
where
    Self: Debug,
{
    fn parse(&self, tokenizer: &mut Tokenizer, grammar: &Grammar, index_start: usize)
        -> RuleStatus;
}

// endregion

// region: ---Token
#[derive(Debug)]
pub struct TokenRule(pub usize); // index of the token definition in the grammar
pub struct TokenContext(usize, String);
impl Debug for TokenContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.1)
    }
}
impl Rule for TokenRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        _grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let token = tokenizer.get_token_from_token_definition(index_start, self.0);
        if let Some(token) = token {
            let context = Context::Token(TokenContext(self.0, token.slice().to_string()));
            RuleStatus::Valid(context, index_start + 1)
        } else {
            RuleStatus::Invalid
        }
    }
}

// endregion

// region: ---Fragment
#[derive(Debug)]
pub struct FragmentRule(pub usize); // index of the fragment in the grammar

pub struct FragmentContext(usize, Box<Context>); // index of the fragment in the grammar and the context of the fragment
impl Debug for FragmentContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.1)
    }
}
impl Rule for FragmentRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let rule: &dyn Rule = grammar.get_rule_of_fragment(self.0);
        let rule_status = rule.parse(tokenizer, grammar, index_start);
        match rule_status {
            RuleStatus::Valid(context, index_end) => RuleStatus::Valid(
                Context::Fragment(FragmentContext(self.0, Box::from(context))),
                index_end,
            ),
            RuleStatus::Invalid => RuleStatus::Invalid,
        }
    }
}

// endregion

// region: ---Sequence
#[derive(Debug)]
pub struct SequenceRule(pub Vec<Box<dyn Rule>>); // index of the fragment in the grammar
#[derive(Debug)]

pub struct SequenceContext(Vec<Context>); // index of the fragment in the grammar and the context of the fragment

impl Rule for SequenceRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let mut index = index_start;

        let mut sequence_context = Vec::with_capacity(self.0.len());

        for rule in &self.0 {
            let rule_status = rule.parse(tokenizer, grammar, index);
            match rule_status {
                RuleStatus::Valid(context, index_end) => {
                    // update index
                    index = index_end;

                    // add context
                    sequence_context.push(context);
                }
                RuleStatus::Invalid => return RuleStatus::Invalid,
            }
        }

        RuleStatus::Valid(Context::Sequence(SequenceContext(sequence_context)), index)
    }
}

// endregion

// region: ---Or
#[derive(Debug)]
pub struct OrRule(pub Vec<Box<dyn Rule>>);
#[derive(Debug)]

pub struct OrContext(usize, Box<Context>);

impl Rule for OrRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        for (index_rule, rule) in self.0.iter().enumerate() {
            let rule_status = rule.parse(tokenizer, grammar, index_start);
            match rule_status {
                RuleStatus::Valid(context, index_end) => {
                    return RuleStatus::Valid(
                        Context::Or(OrContext(index_rule, Box::new(context))),
                        index_end,
                    )
                }
                RuleStatus::Invalid => continue,
            }
        }

        RuleStatus::Invalid
    }
}

// endregion

// region: ---Optional
#[derive(Debug)]
pub struct OptionalRule(pub Box<dyn Rule>);
#[derive(Debug)]

pub struct OptionalContext(Box<Option<Context>>);

impl Rule for OptionalRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let rule_status = self.0.parse(tokenizer, grammar, index_start);
        match rule_status {
            RuleStatus::Valid(context, index_end) => RuleStatus::Valid(
                Context::Optional(OptionalContext(Box::new(Some(context)))),
                index_end,
            ),
            RuleStatus::Invalid => RuleStatus::Valid(
                Context::Optional(OptionalContext(Box::new(None))),
                index_start,
            ),
        }
    }
}

// endregion
