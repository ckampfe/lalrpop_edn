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
    }

    #[test]
    fn keywords() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse(":hello").is_ok());

        assert!(parser.parse(":some_namespace/hello").is_ok());

        assert!(parser.parse(":some_namespace/hello]").is_err());
    }

    #[test]
    fn characters() {
        let parser = edn::ExprParser::new();

        assert!(parser.parse("\\n").is_ok());
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

        assert!(parser.parse("-42N").is_ok());

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

        assert!(parser.parse("-42M").is_ok());

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
}
