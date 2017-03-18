use formula::*;

mod test;

pub trait PropLogic<T> {
    fn eval(&self, v: &mut FnMut(&T) -> bool) -> bool;
}

impl<T> PropLogic<T> for Formula<T> {
    /// eval for propositions, from 2.2
    fn eval(&self, v: &mut FnMut(&T) -> bool) -> bool {
        match *self.kind {
            FormulaKind::False => false,
            FormulaKind::True => true,
            FormulaKind::Atom(ref a) => v(a),
            FormulaKind::Not(ref a) => !a.eval(v),
            FormulaKind::Or(ref a, ref b) => a.eval(v) || b.eval(v),
            FormulaKind::And(ref a, ref b) => a.eval(v) && b.eval(v),
            FormulaKind::Implies(ref a, ref b) => !a.eval(v) || b.eval(v),
            FormulaKind::Iff(ref a, ref b) => a.eval(v) == b.eval(v),
            FormulaKind::ForAll(..) | FormulaKind::Exists(..) => {
                panic!("forall/exists not present in proposotional logic")
            }
        }
    }
}

