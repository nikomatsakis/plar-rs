use lalrpop_intern::InternedString;
use std::sync::Arc;

#[macro_use]
mod macros;
mod debug;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Formula<T> {
    pub kind: Arc<FormulaKind<T>>
}

impl<T> Formula<T> {
    pub fn with(kind: FormulaKind<T>) -> Self {
        Formula { kind: Arc::new(kind) }
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum FormulaKind<T> {
    False,
    True,
    Atom(T),
    Not(Formula<T>),
    And(Formula<T>, Formula<T>),
    Or(Formula<T>, Formula<T>),
    Implies(Formula<T>, Formula<T>),
    Iff(Formula<T>, Formula<T>),
    ForAll(InternedString, Formula<T>),
    Exists(InternedString, Formula<T>),
}
