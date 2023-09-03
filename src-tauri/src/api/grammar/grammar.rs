use std::rc::Rc;

use super::{fragment::Fragment, token::Token};

#[derive(Debug)]
pub struct Grammar<'a> {
    pub tokens: Vec<Rc<Token>>,
    pub fragments: Vec<Rc<Fragment<'a>>>,
}
