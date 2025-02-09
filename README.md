# Serde AT Protocol DAG-JSON

[![Crates.io](https://img.shields.io/crates/v/serde_atproto_dagjson.svg)](https://crates.io/crates/serde_atproto_dagjson)
[![Documentation](https://docs.rs/serde_atproto_dagjson/badge.svg)](https://docs.rs/serde_atproto_dagjson)

This is a [Serde] implementation for [DAG-JSON] for AT protocol.

It is forked from `serde_ipld_dagjson` with a few alterations to use the `bytes`
and `$link` types defined in AT protocol's [data model](https://atproto.com/specs/data-model)
rather than the reserved key `/` defined in IPLD's `DAG-JSON`.

Any Serde based JSON library can be used for the underlying parsing, by default [serde_json] is used.

[Serde]: https://github.com/serde-rs/serde
[DAG-CBOR]: https://ipld.io/specs/codecs/dag-json/spec/
[ipld-core]: https://crates.io/crates/ipld-core
[serde_json]: https://crates.io/crates/serde_json

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
