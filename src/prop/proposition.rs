use std::fmt;

use lalrpop_intern::{intern, InternedString};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Proposition {
    name: InternedString
}

pub fn prop(s: &'static str) -> Proposition {
    Proposition { name: intern(s) }
}

impl fmt::Debug for Proposition {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}
