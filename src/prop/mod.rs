use formula::*;

mod proposition;
mod test;

pub trait PropLogic<T> {
    fn eval(&self, v: &mut FnMut(&T) -> bool) -> bool;
    fn psimplify(&self) -> Formula<T>;
    fn psimplify1(&self) -> Formula<T>;
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
            FormulaKind::ForAll(..) |
            FormulaKind::Exists(..) => panic!("forall/exists not present in prop logic"),
        }
    }

    fn psimplify(&self) -> Formula<T> {
        match *self.kind {
            FormulaKind::Not(ref f) => f.psimplify().not().psimplify1(),
            FormulaKind::And(ref p, ref q) => {
                formula!(and {p.psimplify()} {q.psimplify()}).psimplify1()
            }
            FormulaKind::Or(ref p, ref q) => {
                formula!(or {p.psimplify()} {q.psimplify()}).psimplify1()
            }
            FormulaKind::Implies(ref p, ref q) => {
                formula!(implies {p.psimplify()} {q.psimplify()}).psimplify1()
            }
            FormulaKind::Iff(ref p, ref q) => {
                formula!(iff {p.psimplify()} {q.psimplify()}).psimplify1()
            }
            _ => self.clone(),
        }
    }

    fn psimplify1(&self) -> Formula<T> {
        match *self.kind {
            FormulaKind::False => self.clone(),

            FormulaKind::True => self.clone(),

            FormulaKind::Atom(_) => self.clone(),

            FormulaKind::Not(ref f) => {
                match *f.kind {
                    FormulaKind::False => formula!(true),
                    FormulaKind::True => formula!(false),
                    FormulaKind::Not(ref g) => g.clone(),
                    _ => self.clone(),
                }
            }

            FormulaKind::And(ref f, ref g) => {
                if f.is_false() || g.is_false() {
                    formula!(false)
                } else if f.is_true() {
                    g.clone()
                } else if g.is_true() {
                    f.clone()
                } else {
                    self.clone()
                }
            }

            FormulaKind::Or(ref f, ref g) => {
                if f.is_true() || g.is_true() {
                    formula!(true)
                } else if f.is_false() {
                    g.clone()
                } else if g.is_false() {
                    f.clone()
                } else {
                    self.clone()
                }
            }

            FormulaKind::Implies(ref f, ref g) => {
                if f.is_false() || g.is_true() {
                    formula!(true)
                } else if f.is_true() {
                    g.clone()
                } else if g.is_false() {
                    f.not()
                } else {
                    self.clone()
                }
            }

            FormulaKind::Iff(ref f, ref g) => {
                if f.is_true() {
                    g.clone()
                } else if g.is_true() {
                    f.clone()
                } else if f.is_false() {
                    g.not()
                } else if g.is_false() {
                    f.not()
                } else {
                    self.clone()
                }
            }

            FormulaKind::ForAll(..) |
            FormulaKind::Exists(..) => panic!("forall/exists not present in prop logic"),
        }
    }
}
