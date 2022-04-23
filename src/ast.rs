use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Expr<'input> {
    String(&'input str),
    Symbol {
        namespace: Option<&'input str>,
        name: &'input str,
    },
    Keyword {
        namespace: Option<&'input str>,
        name: &'input str,
    },
    Character(char),
    Integer(Integer),
    Float(Float),
    Nil,
    Comment,
    List(Vec<Expr<'input>>),
    Vector(Vec<Expr<'input>>),
    Set(BTreeSet<Expr<'input>>),
    Map(BTreeMap<Expr<'input>, Expr<'input>>),
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::String(s) => f.write_str(s),
            Expr::Symbol { namespace, name } => {
                if let Some(namespace) = namespace {
                    write!(f, "{}/", namespace)?;
                }

                f.write_str(name)
            }
            Expr::Keyword { namespace, name } => {
                f.write_str(":")?;

                if let Some(namespace) = namespace {
                    write!(f, "{}/", namespace)?;
                }

                f.write_str(name)
            }
            Expr::Character(c) => {
                // This is ugly.
                // Edn/Clojure does chars like: \u1234,
                // Rust does chars like: \u{1234},
                // so we have to write them like edn does, by hand.
                //
                // Further, rust doesn't recognize \b, \backspace, or space,
                // so we have to match them by hand.

                let escaped = c.escape_default().filter(|c| *c != '{' && *c != '}');

                let coll: String = escaped.collect();

                match coll.as_str() {
                    // backspace
                    "\\u8" => f.write_str("\\backspace")?,
                    // space
                    " " => f.write_str("\\space")?,
                    // something else
                    _ => f.write_str(&coll)?,
                }

                Ok(())
            }
            Expr::Integer(i) => match i {
                Integer::Exact(i) => {
                    let mut buffer = itoa::Buffer::new();
                    let printed = buffer.format(*i);
                    f.write_str(printed)
                }
                Integer::Arbitrary(i) => write!(f, "{}N", i),
            },
            Expr::Float(float) => match float {
                Float::Double(d) => {
                    let mut buffer = ryu::Buffer::new();
                    let printed = buffer.format(d.0);
                    f.write_str(printed)
                }
                Float::Exact(d) => {
                    write!(f, "{}M", d)
                }
            },
            Expr::Nil => f.write_str("nil"),
            Expr::List(l) => write_collection(f, l, "(", ")"),
            Expr::Vector(v) => write_collection(f, v, "[", "]"),
            Expr::Set(s) => write_collection(f, s, "#{", "}"),
            Expr::Map(m) => {
                let as_iter = m.into_iter();

                f.write_str("{")?;

                let mut peekable = as_iter.peekable();

                while let Some((k, v)) = peekable.next() {
                    if peekable.peek().is_some() {
                        write!(f, "{} {}, ", k, v)?;
                    } else {
                        write!(f, "{} {}", k, v)?;
                    }
                }

                f.write_str("}")
            }
            Expr::Comment => todo!(),
        }
    }
}

fn write_collection<'a, C>(
    f: &mut std::fmt::Formatter,
    c: C,
    open: &str,
    close: &str,
) -> std::fmt::Result
where
    C: IntoIterator<Item = &'a Expr<'a>>,
{
    let as_iter = c.into_iter();

    f.write_str(open)?;

    let mut peekable = as_iter.peekable();

    while let Some(expr) = peekable.next() {
        if peekable.peek().is_some() {
            write!(f, "{} ", expr)?;
        } else {
            write!(f, "{}", expr)?;
        }
    }

    f.write_str(close)
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Integer {
    Exact(i64),
    Arbitrary(num_bigint::BigInt),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Float {
    Double(ordered_float::OrderedFloat<f64>),
    Exact(rust_decimal::Decimal),
}
