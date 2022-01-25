# lalrpop_edn

A [LALRPOP](https://github.com/lalrpop/lalrpop) parser for [edn](https://github.com/edn-format/edn)

[![Rust](https://github.com/ckampfe/lalrpop_edn/actions/workflows/rust.yml/badge.svg)](https://github.com/ckampfe/lalrpop_edn/actions/workflows/rust.yml)

```rust
let parser = edn::ExprParser::new();

let s = std::fs::read_to_string("deps.edn").unwrap();

assert!(parser.parse(&s).is_ok());
```

## TODO

- [x] Strings
- [x] Symbols
- [x] Keywords
- [x] Characters
- [x] Integers
- [x] Integers (arbitrary precision)
- [x] Floats
- [x] Floats (exact precision)
- [x] Lists
- [x] Vectors
- [x] Maps
- [x] Sets
- [ ] Nil
- [ ] Comments
- [ ] Discard (`#_`)
- [ ] Tagged elements
- [ ] Get rid of all `.unwrap()`/`.expect()`
- [ ] Make `num-bigint` and `rust_decimal` optional
