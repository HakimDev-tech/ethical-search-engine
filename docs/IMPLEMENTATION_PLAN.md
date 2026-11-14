# V0.1 Implementation Plan

- Document ID: ESE-PLAN-001
- Version: 1.0
- Target: V0.1
- Status: Implementation baseline proposal
- Repository: HakimDev-tech/ethical-search-engine
- Default branch: main

## 1. Goal

Implement only the engineering foundation required by the V0.1 product specification. The result must be a small, compiling, tested Rust workspace with clear boundaries and reproducible CI. No crawler, search engine, ranking algorithm, moderation classifier, database, HTTP API or frontend is implemented in this release.

## 2. Locked technical baseline

| Item | Decision |
|---|---|
| Rust toolchain | 1.100.0 stable |
| Rust edition | 2024 |
| Declared MSRV | 1.85.0 (minimum required by edition 2024) |
| Build system | Cargo |
| Workspace style | Virtual Cargo workspace |
| CI platform | GitHub Actions |
| CI runner | Ubuntu latest |
| Local development | Windows, Linux and macOS where Rust supports the target |
| Unsafe Rust | Forbidden in workspace code |
| License | Unselected; no LICENSE file until an explicit decision |

Rust 1.100.0 is pinned for reproducible V0.1 builds. MSRV 1.85.0 is a compatibility policy, not a claim that every future dependency will necessarily support it. Dependency resolution must be checked against the declared MSRV before accepting a dependency update.

## 3. Exact V0.1 workspace

The initial workspace contains exactly three library crates:

```text
ethical-search-engine/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .gitignore
├── README.md
├── docs/
│   ├── PRODUCT_SPEC.md
│   ├── ARCHITECTURE.md
│   ├── TECHNICAL_DECISIONS.md
│   ├── ACCEPTANCE_CRITERIA.md
│   └── IMPLEMENTATION_PLAN.md
├── crates/
│   ├── domain/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── config/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── observability/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── .github/
    └── workflows/
        └── ci.yml
```

No binary application is required in V0.1. No empty placeholder crate or future-only directory is permitted.

### 3.1 ese-domain

Responsibilities:
- Validated `DocumentId` and `SourceId` newtypes.
- `LanguageCode` with Arabic, French, English and an explicit unknown/undetermined state.
- `EthicalDecision` with Allow, Review and Exclude.
- `PolicyVersion` as a validated policy revision identifier.
- Domain validation errors local to this crate.

Constraints:
- Standard library only.
- No serialization dependency in V0.1.
- No database, URL-fetching, HTTP, indexing or AI dependency.
- No I/O, environment access or global mutable state.
- Types must derive only traits justified by their semantics.
- Identifiers must reject empty or whitespace-only values; length and character constraints must be explicit and tested.
- Language codes must have canonical representations: ar, fr, en and und for undetermined language. Parsing behavior must be case-insensitive only if explicitly normalized and tested.

### 3.2 ese-config

Responsibilities:
- Typed application configuration for V0.1.
- Deterministic environment-variable loading.
- Defaults and validation.
- Configuration-specific errors.

Constraints:
- Standard library only for the first implementation.
- No secrets are required by V0.1.
- No configuration file parser until a real file-based requirement exists.
- Environment variables use a documented ESE_ prefix.
- Invalid values fail clearly; no silent fallback for malformed explicit values.
- Tests must isolate environment access and avoid order-dependent global environment mutation.

### 3.3 ese-observability

Responsibilities:
- Initialize structured tracing subscriber.
- Configure log filtering from environment with a documented safe default.
- Prevent duplicate global subscriber initialization from becoming an unexplained panic.
- Document initialization lifecycle and error behavior.

Initial dependencies:
- tracing
- tracing-subscriber with only the fmt and env-filter features.

Constraints:
- No OpenTelemetry exporter in V0.1.
- No file logging or log rotation.
- No raw query, secret or personal-data logging.
- Subscriber initialization must return a typed result or a documented error; do not hide failure with unwrap.

## 4. Dependency graph

```text
ese-domain       (no external dependencies)
ese-config       (no external dependencies)
ese-observability
    ├── tracing
    └── tracing-subscriber
```

The three crates are independent in V0.1. In particular, config and observability must not depend on domain merely to share generic types. Future application composition may depend on all three.

A generic shared errors crate is deliberately excluded. Each crate owns its own error types. An executable application may later use anyhow at its outer boundary if justified.

## 5. Workspace policy

Root Cargo.toml must:
- Declare all three workspace members explicitly.
- Set resolver = "3".
- Define edition = "2024" and rust-version = "1.85" in workspace package metadata.
- Centralize only dependencies genuinely shared by multiple crates.
- Define workspace Rust lint policy with unsafe_code = "forbid".
- Define a documented Clippy warning policy.
- Avoid broad dependency additions for future features.

rust-toolchain.toml pins 1.100.0 and requests rustfmt and clippy components.

Cargo.lock is committed.

## 6. Quality gates and commands

The following commands must succeed from the repository root:

```bash
cargo fmt --all -- --check
cargo metadata --no-deps --format-version 1
cargo check --workspace --all-targets
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
cargo deny check
```

The CI workflow runs the required Rust commands with the pinned toolchain, verifies compilation on the declared Rust 1.85.0 MSRV, and performs a `cargo-deny` audit using the committed `deny.toml`. The dependency audit denies known vulnerability and unsoundness advisories, unmaintained crates, yanked crates, unapproved licenses and unknown dependency sources. Duplicate versions are warnings requiring review.

## 7. Implementation sequence

### Step 0 — Repository preflight
- Confirm default branch and clean repository baseline.
- Confirm the four approved reference documents are present.
- Confirm no license has been selected.
- Create the V0.1 implementation branch before code changes.

### Step 1 — Workspace skeleton
- Add root Cargo.toml.
- Add rust-toolchain.toml.
- Add .gitignore.
- Create the three crate manifests and minimal crate roots.
- Generate and commit Cargo.lock.

Exit gate: Cargo metadata, check and build succeed.

### Step 2 — Domain primitives
- Implement identifiers and validation.
- Implement language and policy enums.
- Implement policy version.
- Add unit tests for valid, invalid, boundary and Unicode cases.

Exit gate: domain tests pass and domain has no external dependencies.

### Step 3 — Configuration
- Define configuration types and documented defaults.
- Implement ESE_-prefixed environment parsing.
- Validate malformed and missing values.
- Add isolated tests for precedence and invalid input.

Exit gate: config tests are deterministic and no secrets are logged.

### Step 4 — Observability
- Implement one-time subscriber initialization with explicit error handling.
- Configure env filter and structured formatting.
- Add tests for pure configuration decisions; avoid brittle tests against global subscriber state.

Exit gate: initialization contract is documented and no sensitive values are emitted.

### Step 5 — CI and repository quality
- Add GitHub Actions workflow.
- Run formatting, metadata, check, build, Clippy, tests and docs.
- Run the cargo-deny license and advisory audit.
- Verify the workflow on the V0.1 branch.
- Review dependency tree, licenses and advisories.

Exit gate: all required CI jobs pass.

### Step 6 — Final review and release
- Compare implementation with all four reference documents.
- Review public APIs and dependency boundaries.
- Resolve every failed mandatory acceptance criterion.
- Produce a validation report with commit SHA and toolchain.
- Merge or tag only after explicit acceptance.

## 8. Explicitly deferred

The following must not enter V0.1:
- Tokio, Reqwest, Scraper and crawler logic.
- Tantivy, BM25 and index schema.
- Axum or a public API.
- PostgreSQL or migrations.
- Aho-Corasick or keyword policy rules.
- Candle, ONNX Runtime or embeddings.
- Ranking formula, trust propagation or quality scoring.
- Frontend, Docker and deployment.
- User accounts, query analytics or personalization.

## 9. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Overengineering the foundation | Keep exactly three small crates |
| Generic error abstractions | Keep errors local to their owning crate |
| Unnecessary dependency growth | Standard library for domain and config |
| Global environment test flakiness | Isolate environment access and serialize tests that must mutate process environment |
| Tracing global-state tests | Test pure configuration separately; keep global initialization tests minimal |
| New toolchain regressions | Pin 1.100.0 and run CI on the exact version |
| Confusing architecture with implementation | Clearly mark all deferred systems as not implemented |

## 10. Completion definition

Implementation planning is complete when:
- The exact crate list and responsibilities are approved.
- Toolchain, edition and MSRV are recorded.
- Dependency boundaries are explicit.
- Commands and release gates are reproducible.
- License remains explicitly unresolved rather than guessed.

V0.1 implementation is complete only when every mandatory criterion in ACCEPTANCE_CRITERIA.md passes and a final validation report is recorded.
