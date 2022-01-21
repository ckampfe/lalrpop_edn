use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub edn);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deps() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        let s = std::fs::read_to_string("deps.edn").unwrap();

        assert!(parser.parse(&mut keywords_cache, &s).is_ok());
    }

    #[test]
    fn strings() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "\"\"").is_ok());
        assert!(parser.parse(&mut keywords_cache, "\"hello\"").is_ok());
    }

    #[test]
    fn symbols() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "hello").is_ok());

        assert!(parser
            .parse(&mut keywords_cache, "some_namespace/hello")
            .is_ok());

        assert!(parser
            .parse(&mut keywords_cache, "some_namespace/hello]")
            .is_err());
    }

    #[test]
    fn keywords() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, ":hello").is_ok());

        assert!(parser
            .parse(&mut keywords_cache, ":some_namespace/hello")
            .is_ok());

        assert!(parser
            .parse(&mut keywords_cache, ":some_namespace/hello]")
            .is_err());
    }

    #[test]
    fn characters() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "\\n").is_ok());
        assert!(parser.parse(&mut keywords_cache, "\\newline").is_ok());

        assert!(parser.parse(&mut keywords_cache, "\\r").is_ok());
        assert!(parser.parse(&mut keywords_cache, "\\return").is_ok());

        assert!(parser.parse(&mut keywords_cache, "\\b").is_ok());
        assert!(parser.parse(&mut keywords_cache, "\\backspace").is_ok());

        assert!(parser.parse(&mut keywords_cache, "\\space").is_ok());

        assert!(parser.parse(&mut keywords_cache, "\\t").is_ok());
        assert!(parser.parse(&mut keywords_cache, "\\tab").is_ok());

        assert!(parser.parse(&mut keywords_cache, "\\u241F").is_ok());
    }

    #[test]
    fn integers() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "42").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+42").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+42N").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42N").is_ok());

        assert!(parser
            .parse(&mut keywords_cache, &(i64::MIN + 1).to_string())
            .is_ok());

        assert!(parser
            .parse(&mut keywords_cache, &(i64::MAX).to_string())
            .is_ok());
    }

    #[test]
    fn floats() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "0.0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+0.0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-0.0").is_ok());

        assert!(parser.parse(&mut keywords_cache, "42.1").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+42.1").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1").is_ok());

        assert!(parser.parse(&mut keywords_cache, "+42M").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42M").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1e10").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1e+10").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1e-10").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1E10").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1E+10").is_ok());

        assert!(parser.parse(&mut keywords_cache, "-42.1E-10").is_ok());

        assert!(parser
            .parse(&mut keywords_cache, &format!("{:e}", f64::MIN))
            .is_ok());

        assert!(parser
            .parse(&mut keywords_cache, &format!("{:e}", f64::MAX))
            .is_ok());
    }

    #[test]
    fn vectors() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "[]").is_ok());
        assert!(parser.parse(&mut keywords_cache, "[[]]").is_ok());
        assert!(parser.parse(&mut keywords_cache, "[[] []]").is_ok());
        assert!(parser.parse(&mut keywords_cache, "[[], []]").is_ok());
        assert!(parser.parse(&mut keywords_cache, "[[], [],]").is_ok());
        assert!(parser.parse(&mut keywords_cache, "[[], [],,,,,]").is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "[1 2 3 :foo hi \"ok\"]")
            .is_ok());
    }

    #[test]
    fn lists() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "()").is_ok());
        assert!(parser.parse(&mut keywords_cache, "(())").is_ok());
        assert!(parser.parse(&mut keywords_cache, "(() ())").is_ok());
        assert!(parser.parse(&mut keywords_cache, "((), ())").is_ok());
        assert!(parser.parse(&mut keywords_cache, "((), (),)").is_ok());
        assert!(parser.parse(&mut keywords_cache, "((), (),,,,,)").is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "(1 2 3 :foo hi \"ok\")")
            .is_ok());
    }

    #[test]
    fn sets() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "#{}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "#{#{}}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "#{#{} #{}}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "#{#{}, #{}}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "#{#{}, #{},}").is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "#{#{}, #{},,,,,}")
            .is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "#{1 2 3 :foo hi \"ok\"}")
            .is_ok());
    }

    #[test]
    fn maps() {
        let parser = edn::ExprParser::new();

        let mut keywords_cache = std::collections::HashSet::new();

        assert!(parser.parse(&mut keywords_cache, "{}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "{[] []}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "{:hi :there}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "{:hi \"ok\"}").is_ok());
        assert!(parser.parse(&mut keywords_cache, "{fine blah}").is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "{[], []          ,}")
            .is_ok());
        assert!(parser.parse(&mut keywords_cache, "{:hi, :there,,}").is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "{:hi,,, ,, \"ok\"}")
            .is_ok());
        assert!(parser
            .parse(&mut keywords_cache, "{fine ,,,,blah,,,,,}")
            .is_ok());
        assert!(parser
            .parse(
                &mut keywords_cache,
                "{
                :a :b\n\n\n
                :c :d
                ,:e :x



    }"
            )
            .is_ok());

        assert!(parser.parse(&mut keywords_cache, "{[]}").is_err());
    }
}
