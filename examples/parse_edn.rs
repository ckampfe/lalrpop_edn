use lalrpop_edn::edn;

fn main() {
    let input = std::fs::read_to_string("deps.edn").unwrap();
    let parser = edn::ExprParser::new();
    let ast = parser.parse(&input).unwrap();
    println!("{:#?}", ast);
}
