# Ethical Search Engine

An independent, privacy-conscious, multilingual web search engine focused on relevance, source credibility, information quality and transparent ethical content policies.

**Product languages:** Arabic, French and English  
**Primary implementation language:** Rust  
**Current implementation baseline:** V0.1 engineering foundation  
**Next release under specification:** V0.2 secure crawler foundation

## Project status

The V0.1 Rust foundation is implemented on `v0.1/implementation` and remains under review in [Pull Request #1](https://github.com/HakimDev-tech/ethical-search-engine/pull/1). It contains `ese-domain`, `ese-config` and `ese-observability`, uses Rust 1.100.0 / Edition 2024, declares MSRV 1.85.0 and commits Cargo.lock.

V0.2 is being specified on `v0.2/specification`. No crawler implementation or network dependency has been added by this specification branch. The planned crawler will accept explicit seed URLs and perform bounded, robots-aware HTML acquisition; it will not recursively crawl or extract/index documents.

## Reference documentation

### Product and architecture
- [Product specification](docs/PRODUCT_SPEC.md)
- [System architecture](docs/ARCHITECTURE.md)
- [Technical decisions](docs/TECHNICAL_DECISIONS.md)

### V0.1
- [V0.1 acceptance criteria](docs/ACCEPTANCE_CRITERIA.md)
- [V0.1 implementation plan](docs/IMPLEMENTATION_PLAN.md)

### V0.2 — Secure crawler foundation
- [Crawler specification](docs/V0.2_CRAWLER_SPEC.md)
- [Crawler architecture](docs/V0.2_CRAWLER_ARCHITECTURE.md)
- [Crawler technical decisions](docs/V0.2_CRAWLER_DECISIONS.md)
- [Crawler acceptance criteria](docs/V0.2_CRAWLER_ACCEPTANCE.md)
- [Crawler implementation plan](docs/V0.2_CRAWLER_IMPLEMENTATION_PLAN.md)
- [Dependency and API preflight](docs/V0.2_DEPENDENCY_PREFLIGHT.md)

## V0.1 development

Install the pinned Rust toolchain with rustup, then run from the repository root:

```bash
cargo fmt --all -- --check
cargo metadata --no-deps --format-version 1
cargo check --workspace --all-targets --locked
cargo build --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo doc --workspace --no-deps --locked
cargo deny check
```

The dependency audit uses the committed `deny.toml`. The application log level is configured through `ESE_LOG_LEVEL`; supported values are `error`, `warn`, `info`, `debug` and `trace`, with `info` as the default.

## Scope discipline

A passing foundation build does not mean the search engine is operational. No extraction, deduplication, index, BM25, ERE ranking, ethical policy engine, public API or frontend is claimed as implemented.

## License

No license has been selected yet. Until a license is added, no open-source reuse permissions should be assumed.
