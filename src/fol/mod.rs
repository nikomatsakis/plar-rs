use itertools::Itertools;
use lalrpop_intern::InternedString;
use std::collections::HashMap;
use std::iter::{empty, once};
use util::IteratorObject;

mod debug;
mod test;

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    Var(InternedString),
    Fn(Apply),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Apply {
    name: InternedString,
    terms: Vec<Term>
}

pub struct Domain {
    constant_terms: Vec<Term>,
    funcs: HashMap<InternedString, usize>,
}

impl Domain {
    pub fn ground_terms(&self, n: usize) -> IteratorObject<Term> {
        if n == 0 {
            IteratorObject::new(self.constant_terms.iter().cloned())
        } else {
            IteratorObject::new(
                self.funcs
                    .iter()
                    .flat_map(move |(&name, &arity)| {
                        self.ground_tuples(n - 1, arity)
                            .map(move |terms| Term::Fn(Apply { name, terms }))
                    }))
        }
    }

    pub fn ground_tuples(&self, n: usize, arity: usize) -> IteratorObject<Vec<Term>> {
        if arity == 0 {
            if n == 0 {
                IteratorObject::new(once(vec![]))
            } else {
                IteratorObject::new(empty())
            }
        } else {
            IteratorObject::new(
                (0...n)
                    .flat_map(move |k| {
                        self.ground_terms(k)
                            .cartesian_product(self.ground_tuples(n - k, arity - 1)
                                               .collect::<Vec<_>>())
                            .map(move |(e, mut v)| {
                                v.push(e);
                                v
                            })
                    }))
        }
    }

}

