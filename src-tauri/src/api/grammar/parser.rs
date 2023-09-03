use std::rc::Rc;

use super::{fragment::Fragment, token::Token};

// region: ---Rule
#[derive(Debug)]
pub enum FragmentRule<'a> {
    Token(Rc<Token>),
    Fragment(Rc<Fragment<'a>>),

    Sequence(Vec<FragmentRule<'a>>),
    Or(Vec<FragmentRule<'a>>),
    Optional(&'a FragmentRule<'a>),
    ZeroOrMore(&'a FragmentRule<'a>),
    OneOrMore(&'a FragmentRule<'a>),
    MinLoop {
        min: u32,
        value: &'a FragmentRule<'a>,
    },
    MinMaxLoop {
        min: u32,
        max: u32,
        value: &'a FragmentRule<'a>,
    },
}

// endregion
