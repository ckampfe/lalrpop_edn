use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub edn);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deps() {
        let parser = edn::ExprParser::new();

        let s = std::fs::read_to_string("deps.edn").unwrap();

        assert!(parser.parse(&s).is_ok());
    }

    #[test]
    fn strings() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("\"\"").is_ok());
        assert!(parser.parse("\"hello\"").is_ok());
    }

    #[test]
    fn symbols() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("hello").is_ok());

        assert!(parser.parse("some_namespace/hello").is_ok());

        assert!(parser.parse("some_namespace/hello]").is_err());

        assert!(parser.parse("422some_namespace/hello]").is_err());

        assert!(parser.parse("some_namespace/9428hello]").is_err());

        assert!(matches!(
            parser.parse("some_namespace/hello").unwrap(),
            ast::Expr::Symbol {
                namespace: Some("some_namespace"),
                name: "hello"
            }
        ))
    }

    #[test]
    fn keywords() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse(":hello").is_ok());

        assert!(parser.parse(":some_namespace/hello").is_ok());

        assert!(parser.parse(":some_namespace/hello]").is_err());

        assert!(parser.parse(":422some_namespace/hello]").is_err());

        assert!(parser.parse(":some_namespace/9428hello]").is_err());

        assert!(matches!(
            parser.parse(":some_namespace/hello").unwrap(),
            ast::Expr::Keyword {
                namespace: Some("some_namespace"),
                name: "hello"
            }
        ))
    }

    #[test]
    fn characters() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("\\n").is_ok());
        assert!(matches!(
            parser.parse("\\n").unwrap(),
            ast::Expr::Character('\n')
        ));
        assert!(parser.parse("\\newline").is_ok());

        assert!(parser.parse("\\r").is_ok());
        assert!(parser.parse("\\return").is_ok());

        assert!(parser.parse("\\b").is_ok());
        assert!(parser.parse("\\backspace").is_ok());

        assert!(parser.parse("\\space").is_ok());

        assert!(parser.parse("\\t").is_ok());
        assert!(parser.parse("\\tab").is_ok());

        assert!(parser.parse("\\u241F").is_ok());
    }

    #[test]
    fn integers() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("0").is_ok());

        assert!(parser.parse("+0").is_ok());

        assert!(parser.parse("-0").is_ok());

        assert!(parser.parse("42").is_ok());

        assert!(parser.parse("+42").is_ok());

        assert!(parser.parse("-42").is_ok());

        assert!(parser.parse("+42N").is_ok());

        assert!(matches!(
            parser.parse("+42N").unwrap(),
            crate::ast::Expr::Integer(crate::ast::Integer::Arbitrary(_))
        ));

        assert!(parser.parse("-42N").is_ok());

        assert!(matches!(
            parser.parse("-42N").unwrap(),
            crate::ast::Expr::Integer(crate::ast::Integer::Arbitrary(_))
        ));

        assert!(parser.parse(&(i64::MIN + 1).to_string()).is_ok());

        assert!(parser.parse(&(i64::MAX).to_string()).is_ok());
    }

    #[test]
    fn floats() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("0.0").is_ok());

        assert!(parser.parse("+0.0").is_ok());

        assert!(parser.parse("-0.0").is_ok());

        assert!(parser.parse("42.1").is_ok());

        assert!(parser.parse("+42.1").is_ok());

        assert!(parser.parse("-42.1").is_ok());

        assert!(parser.parse("+42M").is_ok());

        assert!(matches!(
            parser.parse("+42M").unwrap(),
            crate::ast::Expr::Float(crate::ast::Float::Exact(_))
        ));

        assert!(parser.parse("-42M").is_ok());

        assert!(matches!(
            parser.parse("-42M").unwrap(),
            crate::ast::Expr::Float(crate::ast::Float::Exact(_))
        ));

        assert!(parser.parse("-42.1e10").is_ok());

        assert!(parser.parse("-42.1e+10").is_ok());

        assert!(parser.parse("-42.1e-10").is_ok());

        assert!(parser.parse("-42.1E10").is_ok());

        assert!(parser.parse("-42.1E+10").is_ok());

        assert!(parser.parse("-42.1E-10").is_ok());

        assert!(parser.parse(&format!("{:e}", f64::MIN)).is_ok());

        assert!(parser.parse(&format!("{:e}", f64::MAX)).is_ok());
    }

    #[test]
    fn nil() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("nil").is_ok())
    }
    #[test]
    fn comment() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse(";").is_ok());
        assert!(parser.parse(";;").is_ok());
        assert!(parser.parse(";\n").is_ok());
        assert!(parser.parse(";;\n").is_ok());
        assert!(parser.parse("; ()\n").is_ok());
        assert_eq!(parser.parse("; (1 2 3)").unwrap(), ast::Expr::Comment);
        assert_eq!(parser.parse("; (1 2 3)\n").unwrap(), ast::Expr::Comment);
    }

    #[test]
    fn vectors() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("[]").is_ok());
        assert!(parser.parse("[[]]").is_ok());
        assert!(parser.parse("[[] []]").is_ok());
        assert!(parser.parse("[[], []]").is_ok());
        assert!(parser.parse("[[], [],]").is_ok());
        assert!(parser.parse("[[], [],,,,,]").is_ok());
        assert!(parser.parse("[1 2 3 :foo hi \"ok\"]").is_ok());
    }

    #[test]
    fn lists() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("()").is_ok());
        assert!(parser.parse("(())").is_ok());
        assert!(parser.parse("(() ())").is_ok());
        assert!(parser.parse("((), ())").is_ok());
        assert!(parser.parse("((), (),)").is_ok());
        assert!(parser.parse("((), (),,,,,)").is_ok());
        assert!(parser.parse("(1 2 3 :foo hi \"ok\")").is_ok());
    }

    #[test]
    fn sets() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("#{}").is_ok());
        assert!(parser.parse("#{#{}}").is_ok());
        assert!(parser.parse("#{#{} #{}}").is_ok());
        assert!(parser.parse("#{#{}, #{}}").is_ok());
        assert!(parser.parse("#{#{}, #{},}").is_ok());
        assert!(parser.parse("#{#{}, #{},,,,,}").is_ok());
        assert!(parser.parse("#{1 2 3 :foo hi \"ok\"}").is_ok());
    }

    #[test]
    fn maps() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("{}").is_ok());
        assert!(parser.parse("{[] []}").is_ok());
        assert!(parser.parse("{:hi :there}").is_ok());
        assert!(parser.parse("{:hi \"ok\"}").is_ok());
        assert!(parser.parse("{fine blah}").is_ok());
        assert!(parser.parse("{[], []          ,}").is_ok());
        assert!(parser.parse("{:hi, :there,,}").is_ok());
        assert!(parser.parse("{:hi,,, ,, \"ok\"}").is_ok());
        assert!(parser.parse("{fine ,,,,blah,,,,,}").is_ok());
        assert!(parser
            .parse(
                "{
                :a :b\n\n\n
                :c :d
                ,:e :x



    }"
            )
            .is_ok());

        assert!(parser.parse("{[]}").is_err());
    }

    #[test]
    fn roundtrip() {
        macro_rules! assert_roundtrip {
            ($parser:expr, $input_string:expr) => {
                // parse the given string
                let expr = $parser.parse($input_string).expect("could not parse input string");

                // print it using its Display impl
                let as_string = expr.to_string();

                // parse the printed string
                let parsed = $parser
                    .parse(&as_string)
                    .expect("could not parse printed value");

                // assert that: (result of parsing the printed string) == (result of parsing the given string)
                assert_eq!(expr, parsed)
            };
            ($parser:expr, $($s:expr),*) => {
                $(
                    {
                        assert_roundtrip!($parser, $s);
                    }
                )*
            };
        }

        let parser = edn::ExprParser::new();

        // nil
        assert_roundtrip!(parser, "nil");

        // strings
        assert_roundtrip!(
            parser,
            "\"\"",
            "\"foo\"",
            r#""hello with a \ literal slash""#
        );

        // characters
        assert_roundtrip!(
            parser,
            "\\r",
            "\\return",
            "\\n",
            "\\newline",
            "\\b",
            "\\backspace",
            "\\space",
            "\\t",
            "\\tab",
            "\\u241F"
        );

        // symbols
        assert_roundtrip!(parser, "foo", "my-ns/foo", "-hi", ".ok");

        // keywords
        assert_roundtrip!(parser, ":foo", ":my-ns/foo", ":-hi", ":.ok");

        // integers
        assert_roundtrip!(parser, "0", "+0", "-0", "42", "+42", "-42", "+42N");

        // floats
        assert_roundtrip!(
            parser,
            "13.9",
            "13M",
            "-42.1e10",
            "-42.1e+10",
            "-42.1e-10",
            "-42.1E10",
            "-42.1E+10",
            "-42.1E-10"
        );

        // lists
        assert_roundtrip!(
            parser,
            "()",
            "(1 2 3)",
            "(1,2,3)",
            "(1,2 3,)",
            "(+ 1 2 3)",
            "(.hello 1 2 3)"
        );

        // vectors
        assert_roundtrip!(parser, "[]", "[1 2 3]", "[1,2,3]", "[1,2 3,]", "[- 1 2 3]");

        // sets
        assert_roundtrip!(
            parser,
            "#{}",
            "#{1 2 3}",
            "#{1,2,3}",
            "#{1,2 3,}",
            "#{.hi 1 2 3}"
        );

        // maps
        assert_roundtrip!(
            parser,
            "{}",
            "{1 2 3 4}",
            "{:a 1 :b [true nil 9]}",
            "{:-some key 1 #{nil + -}}"
        );

        // deps.edn
        let deps = include_str!("../deps.edn");
        assert_roundtrip!(parser, deps);
    }
}
