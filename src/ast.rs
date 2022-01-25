use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Expr<'input> {
    String(&'input str),
    Symbol(&'input str),
    Keyword(&'input str),
    Character(char),
    Integer(i64),
    Float(Precision),
    List(Vec<Expr<'input>>),
    Vector(Vec<Expr<'input>>),
    Set(BTreeSet<Expr<'input>>),
    Map(BTreeMap<Expr<'input>, Expr<'input>>),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Precision {
    Double(ordered_float::OrderedFloat<f64>),
    Exact(rust_decimal::Decimal),
}
