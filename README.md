# Ethical Search Engine

An independent, privacy-conscious, multilingual web search engine focused on relevance, source credibility, information quality and transparent ethical content policies.

**Current baseline:** V0.1 — engineering foundation in implementation  
**Product languages:** Arabic, French and English  
**Primary implementation language:** Rust

## Project status

The V0.1 Rust workspace is implemented on the `v0.1/implementation` branch and is under review in [Pull Request #1](https://github.com/HakimDev-tech/ethical-search-engine/pull/1).

The initial foundation contains three library crates:
- `ese-domain`: validated domain identifiers, language codes, ethical decisions and policy versions.
- `ese-config`: typed configuration loaded from `ESE_*` environment variables.
- `ese-observability`: structured tracing initialization.

The workspace is pinned to Rust 1.100.0, uses Edition 2024, declares MSRV 1.85.0 and commits `Cargo.lock`. Automated CI checks formatting, compilation, Clippy, tests and documentation.

A passing foundation build does not mean that the search engine itself is operational. No crawler, index, retrieval, ranking or ethical classification engine is implemented yet.

## Reference documentation

- [Product specification](docs/PRODUCT_SPEC.md)
- [System architecture](docs/ARCHITECTURE.md)
- [Technical decisions](docs/TECHNICAL_DECISIONS.md)
- [V0.1 acceptance criteria](docs/ACCEPTANCE_CRITERIA.md)
- [V0.1 implementation plan](docs/IMPLEMENTATION_PLAN.md)

## V0.1 scope

V0.1 establishes the Cargo workspace, foundational domain types, deterministic configuration, structured logging, tests, CI and reproducible developer setup.

Crawler, indexing, search retrieval, ranking, content classification, API and frontend remain future capabilities.

## Development

Install the pinned Rust toolchain with rustup, then run these commands from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo doc --workspace --no-deps --locked
```

The application log level is configured through `ESE_LOG_LEVEL`. Supported values are `error`, `warn`, `info`, `debug` and `trace`; the default is `info`.

## License

No license has been selected yet. Until a license is added, no open-source reuse permissions should be assumed.
