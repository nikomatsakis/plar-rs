#![cfg(test)]
#![allow(non_snake_case)]

use lalrpop_intern::intern;
use super::*;
use super::Term::*;

#[test]
fn ground_terms_zero() {
    let A = intern("A");
    let B = intern("B");
    let F = intern("F");
    let G = intern("G");
    let domain = Domain {
        constant_terms: vec![Var(A), Var(B)],
        funcs: vec![(F, 2), (G, 1)].into_iter().collect(),
    };

    let terms: Vec<_> = domain.ground_terms(0).collect();
    assert_eq!(format!("{:?}", terms),
               format!("[A, B]"));

    let terms: Vec<_> = domain.ground_terms(1).collect();
    assert_eq!(format!("{:?}", terms),
               format!("[F(A, A), F(B, A), F(A, B), F(B, B), G(A), G(B)]"));

    let terms: Vec<_> = domain.ground_terms(2).collect();
    assert_eq!(format!("{:?}", terms),
               format!("[F(F(A, A), A), F(F(B, A), A), F(F(A, B), A), F(F(B, B), A), \
                         F(G(A), A), F(G(B), A), \
                         F(F(A, A), B), F(F(B, A), B), F(F(A, B), B), F(F(B, B), B), \
                         F(G(A), B), F(G(B), B), \
                         F(A, F(A, A)), F(B, F(A, A)), F(A, F(B, A)), F(B, F(B, A)), F(A, F(A, B)), \
                         F(B, F(A, B)), F(A, F(B, B)), F(B, F(B, B)), \
                         F(A, G(A)), F(B, G(A)), F(A, G(B)), F(B, G(B)), \
                         G(F(A, A)), G(F(B, A)), G(F(A, B)), G(F(B, B)), \
                         G(G(A)), G(G(B))]"));
}
