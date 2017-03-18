#![cfg(test)]

use lalrpop_intern::{intern, InternedString};
use super::PropLogic;

macro_rules! valuation {
    ($($f:ident -> $v:expr),*) => {
        &mut |&x: &InternedString| -> bool {
            $(
                if x == $f { return $v; }
            )*
                panic!("unhandled case: {:?}", x);
        }
    }
}

#[test]
fn eval_basics() {
    let p = intern("p");
    let q = intern("q");
    let r = intern("r");
    let f = formula!((implies
                      (and (atom p) (atom q))
                      (and (atom q) (atom r))));
    assert!(f.eval(valuation!(p -> true, q -> false, r -> true)));
    assert!(!f.eval(valuation!(p -> true, q -> true, r -> false)));
}

