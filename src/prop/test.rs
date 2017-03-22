#![cfg(test)]

use super::PropLogic;
use super::proposition::{prop, Proposition};

macro_rules! valuation {
    ($($f:ident -> $v:expr),*) => {
        &mut |&x: &Proposition| -> bool {
            $(
                if x == $f { return $v; }
            )*
                panic!("unhandled case: {:?}", x);
        }
    }
}

#[test]
fn eval_basics() {
    let p = prop("p");
    let q = prop("q");
    let r = prop("r");
    let f = formula!((implies (and [p] [q]) (and [q] [r])));
    assert!(f.eval(valuation!(p -> true, q -> false, r -> true)));
    assert!(!f.eval(valuation!(p -> true, q -> true, r -> false)));
}

#[test]
fn psimplify_true() {
    let p = prop("p");
    let q = prop("q");
    let r = prop("r");
    let f = formula!((or (implies (implies [p] [q]) true) (not false)));
    let g = f.psimplify();
    assert_eq!(format!("{:?}", g), "true");
}

#[test]
fn psimplify_page50() {
    let x = prop("x");
    let y = prop("y");
    let z = prop("r");
    let f = formula!(implies
                     (implies true (iff [x] false))
                     (not (or [y] (and false [z]))));
    let g = f.psimplify();
    assert_eq!(format!("{:?}", g), "(implies (not [x]) (not [y]))");
}

