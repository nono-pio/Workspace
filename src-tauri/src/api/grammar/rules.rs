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
    Loop(LoopContext),
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::Token(token_context) => write!(f, "{:?}", token_context),
            Context::Fragment(fragment_context) => write!(f, "{:?}", fragment_context),
            Context::Sequence(sequence_context) => write!(f, "{:?}", sequence_context),
            Context::Or(or_context) => write!(f, "{:?}", or_context),
            Context::Optional(optional_context) => write!(f, "{:?}", optional_context),
            Context::Loop(loop_context) => write!(f, "{:?}", loop_context),
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
pub struct SequenceContext(Vec<Context>); // index of the fragment in the grammar and the context of the fragment

impl Debug for SequenceContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

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

// region: ---Loop
#[derive(Debug)]
pub struct LoopRule {
    pub rule: Box<dyn Rule>,
    pub separator: Option<Box<dyn Rule>>,
    pub min: usize,
    pub max: usize,
}
#[derive(Debug)]
pub struct LoopContext(
    Vec<Context>,         /*Values*/
    Option<Vec<Context>>, /*Separators*/
);

impl LoopRule {
    pub fn new_min(rule: Box<dyn Rule>, min: usize, separator: Option<Box<dyn Rule>>) -> LoopRule {
        LoopRule {
            rule,
            min,
            max: usize::MAX,
            separator,
        }
    }

    pub fn new_min_max(
        rule: Box<dyn Rule>,
        min: usize,
        max: usize,
        separator: Option<Box<dyn Rule>>,
    ) -> LoopRule {
        LoopRule {
            rule,
            min,
            max,
            separator,
        }
    }

    pub fn new_max(rule: Box<dyn Rule>, max: usize, separator: Option<Box<dyn Rule>>) -> LoopRule {
        LoopRule {
            rule,
            min: 0,
            max,
            separator,
        }
    }

    pub fn new_zero_or_more(rule: Box<dyn Rule>, separator: Option<Box<dyn Rule>>) -> LoopRule {
        LoopRule {
            rule,
            min: 0,
            max: usize::MAX,
            separator,
        }
    }

    pub fn new_one_or_more(rule: Box<dyn Rule>, separator: Option<Box<dyn Rule>>) -> LoopRule {
        LoopRule {
            rule,
            min: 1,
            max: usize::MAX,
            separator,
        }
    }

    fn parse_without_separator(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let mut context_values = Vec::new();
        let mut index = index_start;

        while context_values.len() < self.max {
            let rule_status = self.rule.parse(tokenizer, grammar, index);
            match rule_status {
                RuleStatus::Valid(context, index_end) => {
                    index = index_end;
                    context_values.push(context);
                }
                RuleStatus::Invalid => break,
            }
        }

        if context_values.len() < self.min {
            return RuleStatus::Invalid;
        }

        RuleStatus::Valid(Context::Loop(LoopContext(context_values, None)), index)
    }

    pub fn accept_empty(&self) -> bool {
        self.min == 0
    }

    fn parse_with_separator(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        let mut context_values = Vec::new();
        let mut context_separators = Vec::new();
        let mut index = index_start;

        let separator = self.separator.as_ref().unwrap();

        // parse first value
        let rule_status = self.rule.parse(tokenizer, grammar, index);
        match rule_status {
            RuleStatus::Valid(context, index_end) => {
                index = index_end;
                context_values.push(context);
            }
            RuleStatus::Invalid => {
                return if self.accept_empty() {
                    RuleStatus::Valid(
                        Context::Loop(LoopContext(context_values, Some(context_separators))),
                        index,
                    )
                } else {
                    RuleStatus::Invalid
                }
            }
        }

        while context_values.len() < self.max {
            let separator_index_end: usize;
            let separator_context: Context;

            // parse separator
            let rule_status = separator.parse(tokenizer, grammar, index);
            match rule_status {
                RuleStatus::Valid(context, index_end) => {
                    separator_index_end = index_end;
                    separator_context = context;
                }
                RuleStatus::Invalid => break,
            }
            // parse value
            let rule_status = self.rule.parse(tokenizer, grammar, separator_index_end);
            match rule_status {
                RuleStatus::Valid(context, index_end) => {
                    index = index_end;
                    context_values.push(context);
                }
                RuleStatus::Invalid => break,
            }

            // add separator if value after
            context_separators.push(separator_context);
        }

        if context_values.len() < self.min {
            return RuleStatus::Invalid;
        }

        RuleStatus::Valid(
            Context::Loop(LoopContext(context_values, Some(context_separators))),
            index,
        )
    }
}
impl Rule for LoopRule {
    fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        grammar: &Grammar,
        index_start: usize,
    ) -> RuleStatus {
        if self.separator.is_some() {
            self.parse_with_separator(tokenizer, grammar, index_start)
        } else {
            self.parse_without_separator(tokenizer, grammar, index_start)
        }
    }
}

// endregion
