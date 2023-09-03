// region: ---Fragment

use super::parser::FragmentRule;

#[derive(Debug)]
pub struct Fragment<'a> {
    name: &'a str,
    rule: Option<FragmentRule<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new(name: &'a str) -> Self {
        Fragment { name, rule: None }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_rule(&self) -> &Option<FragmentRule> {
        &self.rule
    }

    pub fn set_rule(&mut self, rule: FragmentRule<'a>) {
        self.rule = Some(rule);
    }
}

// endregion
