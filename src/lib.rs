use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub edn);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deps() {
        let mut keywords_cache = std::collections::HashSet::new();

        let s = std::fs::read_to_string("deps.edn").unwrap();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, &s)
            .is_ok());
    }

    #[test]
    fn strings() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\"\"")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\"hello\"")
            .is_ok());
    }

    #[test]
    fn symbols() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "hello")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "some_namespace/hello")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "some_namespace/hello]")
            .is_err());
    }

    #[test]
    fn keywords() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, ":hello")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, ":some_namespace/hello")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, ":some_namespace/hello]")
            .is_err());
    }

    #[test]
    fn characters() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\n")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\newline")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\r")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\return")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\b")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\backspace")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\space")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\t")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\tab")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "\\u241F")
            .is_ok());
    }

    #[test]
    fn integers() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "42")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+42")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+42N")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42N")
            .is_ok());
    }

    #[test]
    fn floats() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "0.0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+0.0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-0.0")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "42.1")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+42.1")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "+42M")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42M")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1e10")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1e+10")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1e-10")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1E10")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1E+10")
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "-42.1E-10")
            .is_ok());
    }

    #[test]
    fn vectors() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[]")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[[]]")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[[] []]")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[[], []]")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[[], [],]")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "[[], [],,,,,]")
            .is_ok());
    }

    #[test]
    fn lists() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "()")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "(())")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "(() ())")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "((), ())")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "((), (),)")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "((), (),,,,,)")
            .is_ok());
    }

    #[test]
    fn sets() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{#{}}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{#{} #{}}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{#{}, #{}}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{#{}, #{},}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "#{#{}, #{},,,,,}")
            .is_ok());
    }

    #[test]
    fn maps() {
        let mut keywords_cache = std::collections::HashSet::new();

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{[] []}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{:hi :there}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{:hi \"ok\"}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{fine blah}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{[], []          ,}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{:hi, :there,,}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{:hi,,, ,, \"ok\"}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{fine ,,,,blah,,,,,}")
            .is_ok());
        assert!(edn::ExprParser::new()
            .parse(
                &mut keywords_cache,
                "{
                :a :b\n\n\n
                :c :d
                ,:e :x



    }"
            )
            .is_ok());

        assert!(edn::ExprParser::new()
            .parse(&mut keywords_cache, "{[]}")
            .is_err());
    }
}
