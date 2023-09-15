// region: ---Fragment

use crate::api::grammar::rules::Rule;

#[derive(Debug)]
pub struct Fragment {
    name: Box<str>,
    rule: Box<dyn Rule>,
}

impl Fragment {
    pub fn new(name: &str, rule: Box<dyn Rule>) -> Self {
        Fragment {
            name: Box::from(name),
            rule,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_rule(&self) -> &dyn Rule {
        self.rule.as_ref()
    }
}

// endregion
