use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Expr<'input> {
    String(&'input str),
    Symbol(&'input str),
    Keyword(string_cache::DefaultAtom),
    Character(char),
    Integer(i64),
    List(Vec<Expr<'input>>),
    Vector(Vec<Expr<'input>>),
    Set(BTreeSet<Expr<'input>>),
    Map(BTreeMap<Expr<'input>, Expr<'input>>),
}
