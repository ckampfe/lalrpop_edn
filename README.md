# lalrpop_edn

A [LALRPOP](https://github.com/lalrpop/lalrpop) parser for [edn](https://github.com/edn-format/edn)

```rust
let parser = edn::ExprParser::new();

let s = std::fs::read_to_string("deps.edn").unwrap();

assert!(parser.parse(&s).is_ok());
```

## TODO

- [x] String
- [x] Symbols
- [x] Keywords
- [x] Characters
- [x] Integers
- [ ] Integers (arbitrary precision)
- [x] Floats
- [ ] Floats (exact precision)
- [x] Lists
- [x] Vectors
- [x] Maps
- [x] Sets
- [ ] Comments
- [ ] Discard (`#_`)
- [ ] Tagged elements