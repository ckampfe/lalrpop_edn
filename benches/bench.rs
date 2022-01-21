#[macro_use]
extern crate criterion;

use criterion::Criterion;
use lalrpop_edn::*;

fn deps_edn(c: &mut Criterion) {
    let edn = include_str!("../deps.edn");

    c.bench_function("deps.edn", move |b| {
        let mut keywords_cache = std::collections::HashSet::new();
        let parser = edn::ExprParser::new();
        b.iter(|| parser.parse(&mut keywords_cache, edn))
    });
}

criterion_group!(benches, deps_edn);
criterion_main!(benches);
