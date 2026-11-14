# System Architecture — Ethical Search Engine

- Document ID: ESE-ARCH-001
- Document version: 1.1
- Target release: V0.1
- Status: Baseline architecture

## 1. Architectural style

The initial implementation MUST use a modular monolith organized as a Cargo workspace.

This choice provides explicit module boundaries and independently testable components without introducing distributed-system complexity before it is justified. Microservices, message brokers and mandatory external services are out of scope for V0.1.

The architecture separates:
- Domain: product concepts and invariants.
- Application: orchestration of use cases.
- Infrastructure: filesystem, database, search index and network adapters.
- Applications: executable entry points and external interfaces.

Dependencies must point inward. Domain logic must not depend on infrastructure frameworks.

## 2. Technology baseline

| Concern | Selected technology | V0.1 status |
|---|---|---|
| Language | Rust | Required |
| Build and package management | Cargo | Required |
| Async runtime | Tokio | Deferred until needed |
| HTTP client | Reqwest | Future crawler |
| HTML parsing | Scraper | Future extraction |
| Full-text search | Tantivy | Future indexing/retrieval |
| HTTP server | Axum | Future search API |
| Serialization | Serde | Add only where required |
| Database | PostgreSQL | Future persistence |
| Fast multi-pattern matching | Aho-Corasick | Future policy implementation |
| Logging and tracing | tracing | V0.1 foundation |
| Telemetry | OpenTelemetry | Later, when operational needs justify it |
| Semantic inference | Candle or ONNX Runtime | Experimental future capability |
| Frontend | Next.js | Future interface |
| Packaging | Docker | Future deployment target |

Dependencies MUST be introduced only when an implemented V0.1 capability needs them. A technology appearing in this table is not permission to add it prematurely.

## 3. Repository layout

The intended repository layout is:

```text
ethical-search-engine/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .gitignore
├── README.md
├── LICENSE
├── docs/
│   ├── PRODUCT_SPEC.md
│   ├── ARCHITECTURE.md
│   ├── TECHNICAL_DECISIONS.md
│   └── ACCEPTANCE_CRITERIA.md
├── apps/
│   └── (future executable applications)
├── crates/
│   ├── domain/
│   ├── config/
│   ├── errors/
│   └── observability/
├── tests/
│   ├── integration/
│   └── fixtures/
└── .github/
    └── workflows/
        └── ci.yml
```

V0.1 MUST create only the crates that contain real, tested functionality. Future crates must not be created as empty placeholders merely to make the directory tree look complete. The initial workspace may omit future directories until their implementation release.

The repository must contain a license file only after the license choice has been explicitly resolved.

## 4. Dependency boundaries

### 4.1 Domain
Owns stable product types, identifiers, value objects and invariants.

Must not depend on:
- Axum or another HTTP framework.
- Tantivy.
- PostgreSQL clients.
- Reqwest.
- Filesystem-specific persistence.
- AI model runtimes.

### 4.2 Configuration
Owns configuration loading, precedence, parsing and validation. It may depend on serialization or environment libraries where justified, but must not leak secrets through errors or logs.

### 4.3 Errors
Defines shared error conventions and safe diagnostic boundaries. Subsystems may retain typed internal errors and convert them at application boundaries.

### 4.4 Observability
Owns tracing initialization and consistent structured event conventions. It must not introduce raw query logging or secret logging by default.

### 4.5 Applications
Own executable composition and dependency wiring. They may depend on application, domain and infrastructure crates. Domain crates must not depend on application binaries.

## 5. Initial domain primitives

V0.1 should introduce only primitives that can be validated without pretending future subsystems already exist.

### DocumentId
A validated, stable identifier for a document. Its representation must be documented and must not be coupled to a database-generated identifier.

### SourceId
A validated identifier for a source record. It must not itself imply a trust level.

### LanguageCode
A language value supporting at least:
- Arabic: `ar`
- French: `fr`
- English: `en`
- Unknown/undetermined state where appropriate.

The type must not claim to perform language detection.

### EthicalDecision
An enum with:
- `Allow`
- `Review`
- `Exclude`

The unresolved `Review` state must not be interpreted as approved for indexing.

### PolicyVersion
A validated identifier for a policy revision. A policy decision must be attributable to a specific version.

Avoid speculative fields and premature domain entities. Add a type only when its invariants and use are specified.

## 6. Configuration model

Configuration precedence:
1. Explicit environment variables.
2. Configuration file, if one is introduced.
3. Documented defaults.

All configuration must be validated at startup. Invalid configuration must fail clearly and early.

Secrets must:
- Never be committed.
- Never be printed in configuration errors.
- Never be included in ordinary tracing events.
- Be supplied through an appropriate local or deployment secret mechanism.

## 7. Error model

Errors should distinguish at least:
- Invalid input or domain validation.
- Configuration.
- Serialization/deserialization.
- Internal failures.

Future subsystems may add network, parsing, storage and indexing error categories when those capabilities are implemented.

Do not erase diagnostic context with generic strings. Do not expose internal details or secrets through public-facing error messages.

## 8. Testing architecture

- Unit tests validate domain invariants and configuration behavior.
- Integration tests validate interactions between actual workspace components.
- Architecture checks validate forbidden dependency directions.
- Tests must be deterministic and must not require public network access.
- Fixtures must be small, explicit and version-controlled where appropriate.

## 9. CI architecture

The CI pipeline must check:
- Formatting.
- Workspace compilation.
- Clippy.
- Unit and integration tests.
- Documentation build.
- Dependency/security checks where supported.

The exact commands must be committed in the repository and run in CI, not merely listed in documentation.

## 10. Future crawler pipeline

The planned acquisition pipeline is:

```text
URL Discovery
  -> URL and DNS validation
  -> robots.txt and crawl policy
  -> HTTP fetch
  -> HTML extraction
  -> normalization and canonicalization
  -> deduplication
  -> ethical classification
  -> document evaluation
  -> index submission
```

The future crawler must defend against SSRF, private and reserved IP ranges, DNS rebinding, unsafe redirects, non-HTTP schemes, oversized responses, decompression bombs, infinite URL spaces and unbounded concurrency. It must use explicit timeouts, per-host limits, a descriptive User-Agent and documented crawl policies.

robots.txt is a crawl convention, not an authorization mechanism or a substitute for terms-of-service review.

## 11. Future search pipeline

```text
Query
  -> normalization and language processing
  -> candidate retrieval
  -> policy eligibility
  -> relevance ranking
  -> result diversification
  -> explanation and response
```

Policy eligibility is a hard gate according to the active policy. It must not be a ranking bonus that a high relevance score can offset.

Ranking signals such as lexical relevance, source credibility, information quality, freshness and intent relevance must be normalized and evaluated before combination. Privacy is a system property, not a document score.

## 12. Data consistency

When Tantivy and a relational store are introduced, the design must define:
- Which store is authoritative for each field.
- How indexing updates are retried.
- How deleted or excluded documents are removed from searchable indexes.
- How partial failures are detected and repaired.
- How index schema versions and rebuilds are managed.

No dual-write assumption may be left undocumented.

## 13. Architecture changes

A change to a major dependency, crate boundary, data ownership rule or policy boundary requires:
- A written rationale.
- Alternatives considered.
- Migration and compatibility impact.
- Updated tests.
- Updated decision records and documentation.

The architecture is a controlled baseline and may evolve when evidence justifies it.


---

## V0.1 implementation profile — normative clarification (version 1.1)

This section makes the initial workspace concrete and supersedes any earlier generic crate-layout examples in this document.

### Exact initial crates

V0.1 contains exactly these three library crates:

- `ese-domain` at `crates/domain`: identifiers, language values, ethical decision values, policy version and domain validation.
- `ese-config` at `crates/config`: typed configuration, ESE_-prefixed environment parsing, defaults and validation.
- `ese-observability` at `crates/observability`: tracing subscriber initialization and safe structured logging.

There is no generic `errors` crate. Errors belong to the crate that owns the operation and its invariants. A future executable may introduce an application-level error aggregation strategy at its outer boundary.

There is no binary application in V0.1. No empty placeholder crates or future-only directories are permitted.

### Dependency graph

- `ese-domain`: standard library only.
- `ese-config`: standard library only.
- `ese-observability`: `tracing` and `tracing-subscriber` with only the `fmt` and `env-filter` features.

These crates are independent in V0.1. No crate may depend on another merely to share generic error or utility types.

### Toolchain and workspace

- Rust toolchain: 1.100.0, pinned in `rust-toolchain.toml`.
- Edition: 2024.
- Declared MSRV: 1.85.0.
- Cargo resolver: 3.
- `unsafe_code = "forbid"` across workspace code.
- Cargo.lock is committed.
- Formatting, check, Clippy, tests and documentation commands are required in CI.

The toolchain pin is the V0.1 reproducibility baseline. The MSRV is a compatibility policy and must be revalidated against every accepted dependency update.

### Configuration constraints

V0.1 uses environment variables with the `ESE_` prefix and documented defaults. A configuration file parser is deferred until an actual requirement exists. Malformed explicit values must fail rather than silently falling back.

### Deferred systems

Tokio, Reqwest, Scraper, Tantivy, Axum, PostgreSQL, Aho-Corasick, Candle and ONNX Runtime remain future dependencies. Their mention elsewhere in this document describes future architecture only.
