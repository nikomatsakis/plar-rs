macro_rules! formula {
    (($($t:tt)*)) => { formula!($($t)*) };
    ({$e:expr}) => { $e };
    ([$e:expr]) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Atom($e)) };
    (forall<$t:ident> $f:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::ForAll($crate::lalrpop_intern::intern(stringify!($t)), formula!($f))) };
    (exists<$t:ident> $f:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Exists($crate::lalrpop_intern::intern(stringify!($t)), formula!($f))) };
    (false) => { $crate::formula::Formula::with($crate::formula::FormulaKind::False) };
    (true) => { $crate::formula::Formula::with($crate::formula::FormulaKind::True) };
    (not $f:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Not(formula!($f))) };
    (and $f:tt $g:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::And(formula!($f), formula!($g))) };
    (or $f:tt $g:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Or(formula!($f), formula!($g))) };
    (implies $f:tt $g:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Implies(formula!($f), formula!($g))) };
    (iff $f:tt $g:tt) => { $crate::formula::Formula::with($crate::formula::FormulaKind::Iff(formula!($f), formula!($g))) };
}

#[test]
fn build_formula() {
    let f = formula!(and (not [22]) (forall<N> (exists<M> [44])));
    let s = format!("{:?}", f);
    assert_eq!(s, "(and (not [22]) (forall<N> (exists<M> [44])))");
}
