#[macro_use]
extern crate criterion;

use criterion::Criterion;
use lalrpop_edn::*;

fn deps_edn_parse(c: &mut Criterion) {
    let edn = include_str!("../deps.edn");
    let parser = edn::ExprParser::new();
    parser.parse(edn).unwrap();

    c.bench_function("deps.edn parse", move |b| b.iter(|| parser.parse(edn)));
}

fn deps_edn_to_string(c: &mut Criterion) {
    let edn = include_str!("../deps.edn");
    let parser = edn::ExprParser::new();
    let expr = parser.parse(edn).unwrap();

    c.bench_function("deps.edn to_string", move |b| b.iter(|| expr.to_string()));
}

criterion_group!(benches, deps_edn_parse, deps_edn_to_string);
criterion_main!(benches);
