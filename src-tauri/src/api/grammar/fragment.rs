// region: ---Fragment

#[derive(Debug)]
pub struct Fragment {
    name: Box<str>,
    rule_index: usize,
}

impl Fragment {
    pub fn new(name: &str, rule_index: usize) -> Self {
        Fragment {
            name: Box::from(name),
            rule_index,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_rule_index(&self) -> usize {
        self.rule_index
    }
}

// endregion
