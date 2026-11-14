# Acceptance Criteria — Ethical Search Engine V0.1

- Document ID: ESE-AC-001
- Document version: 1.1
- Target release: V0.1
- Status: Baseline acceptance contract

## 1. Acceptance rules

V0.1 is accepted only when every mandatory criterion below passes.

The existence of code, a successful local demonstration or an implementation agent's claim is not evidence of acceptance by itself. Results must be reproducible from the repository.

Criterion status values:
- PASS: verified with evidence.
- FAIL: verified and not satisfied.
- BLOCKED: cannot yet be evaluated because a prerequisite is missing.
- NOT APPLICABLE: explicitly justified; must not be used to hide an incomplete mandatory requirement.

No mandatory criterion may be silently waived.

## 2. Workspace and toolchain

### AC-001 — Cargo workspace
- [ ] A root Cargo.toml defines a valid workspace.
- [ ] Workspace members are explicit and contain real functionality.
- [ ] Shared dependency versions are centralized where that reduces drift.
- [ ] Cargo metadata succeeds.
- [ ] No empty placeholder crates are included.

### AC-002 — Reproducible toolchain
- [ ] rust-toolchain.toml pins a documented stable toolchain or an explicitly justified channel.
- [ ] Required components and targets are documented.
- [ ] Toolchain installation is reproducible on a clean environment.
- [ ] MSRV policy is documented; if no separate MSRV is declared, the pinned toolchain is the minimum supported baseline for V0.1.

### AC-003 — Build
- [ ] cargo check --workspace succeeds.
- [ ] cargo build --workspace succeeds.
- [ ] No unexplained compiler warnings remain.
- [ ] The commands work from the repository root.

## 3. Domain model

### AC-004 — Identifiers and language values
- [ ] DocumentId has a documented representation and validates invalid values.
- [ ] SourceId has a documented representation and validates invalid values.
- [ ] LanguageCode represents Arabic, French and English.
- [ ] An unknown or undetermined language can be represented where required.
- [ ] Unicode text is preserved without lossy conversion.
- [ ] Domain types do not perform language detection or claim linguistic processing that is not implemented.

### AC-005 — Policy outcome
- [ ] EthicalDecision represents Allow, Review and Exclude.
- [ ] Review is not treated as approved.
- [ ] PolicyVersion is attributable to a specific revision.
- [ ] Invalid states cannot be constructed through the supported public API.

### AC-006 — Domain independence
- [ ] Domain does not depend on Axum, Tantivy, PostgreSQL clients, Reqwest or AI runtimes.
- [ ] Domain invariants are tested.
- [ ] Public types have intentional visibility and documented behavior.

## 4. Configuration

### AC-007 — Configuration loading and validation
- [ ] Configuration loads from the documented source.
- [ ] Precedence is deterministic and documented.
- [ ] Invalid values produce actionable errors.
- [ ] Defaults are explicit and tested.
- [ ] Configuration validation occurs before application startup proceeds.

### AC-008 — Secret handling
- [ ] No credentials or private tokens are committed.
- [ ] Secrets are not included in debug output or error messages.
- [ ] Secrets are not emitted in tracing events.
- [ ] .gitignore excludes relevant local secret files where applicable.

## 5. Errors and observability

### AC-009 — Error conventions
- [ ] Shared error categories are documented.
- [ ] Validation errors can be distinguished from internal failures.
- [ ] Diagnostic context is preserved.
- [ ] Production paths do not use unexplained unwrap or expect.
- [ ] Errors do not expose secrets.

### AC-010 — Structured logging
- [ ] Logging initializes successfully.
- [ ] Log level is configurable.
- [ ] Events use consistent structured fields.
- [ ] Raw queries and sensitive personal data are not logged by default.
- [ ] Startup failures can be diagnosed without exposing secrets.

## 6. Architecture and maintainability

### AC-011 — Dependency boundaries
- [ ] Crate dependency direction follows ARCHITECTURE.md.
- [ ] No circular crate dependency exists.
- [ ] Domain remains infrastructure-independent.
- [ ] Application composition is kept outside domain logic.
- [ ] Forbidden dependencies are checked automatically where practical.

### AC-012 — Scope discipline
- [ ] No crawler is represented as functional.
- [ ] No search index or BM25 capability is represented as functional.
- [ ] No ERE ranking or policy classifier is represented as functional.
- [ ] No production API or frontend is represented as functional.
- [ ] No dependency is added solely for a future feature.

### AC-013 — Code quality
- [ ] Public APIs have clear contracts.
- [ ] Unsafe Rust is not used unless a documented safety case and review exist.
- [ ] No unexplained global mutable state exists.
- [ ] No dead code or placeholder implementation is presented as completed functionality.
- [ ] Comments explain non-obvious constraints rather than restating code.

## 7. Testing

### AC-014 — Unit tests
- [ ] Domain validation tests pass.
- [ ] Configuration tests pass.
- [ ] Error behavior tests pass.
- [ ] Serialization tests pass where serialization is implemented.
- [ ] Important edge cases are covered.

### AC-015 — Integration tests
- [ ] Integration tests run from the workspace.
- [ ] Tests are deterministic.
- [ ] Tests do not require public network access.
- [ ] Failure paths are tested.
- [ ] Test fixtures are explicit and controlled.

### AC-016 — Regression protection
- [ ] Test commands are documented.
- [ ] CI executes the same relevant test suites.
- [ ] A failed test causes CI failure.

## 8. Documentation

### AC-017 — Reference documents
- [ ] docs/PRODUCT_SPEC.md exists and matches the agreed product decisions.
- [ ] docs/ARCHITECTURE.md matches the actual crate structure.
- [ ] docs/TECHNICAL_DECISIONS.md records the selected and deferred technologies.
- [ ] docs/ACCEPTANCE_CRITERIA.md contains verifiable release requirements.
- [ ] README.md provides clean setup, build, test and lint instructions.
- [ ] Documentation does not claim unimplemented capabilities.

## 9. CI and dependency security

### AC-018 — CI
- [ ] Formatting validation runs.
- [ ] Workspace compilation runs.
- [ ] Clippy runs with the documented warning policy.
- [ ] Tests run.
- [ ] Documentation checks run.
- [ ] CI reports failure when a required check fails.

### AC-019 — Dependency review
- [ ] Direct dependencies have documented purposes.
- [ ] Dependency licenses are reviewed for compatibility.
- [ ] Known security advisories are checked using an available tool.
- [ ] Any unresolved advisory is recorded with severity and disposition.
- [ ] Cargo.lock is committed.

## 10. Release gate

V0.1 may be marked ACCEPTED only when:
1. All mandatory criteria pass.
2. CI succeeds on the release commit.
3. The architecture matches the implementation.
4. Documentation matches the repository.
5. No unresolved critical or high-severity security issue remains.
6. The final review report is committed or attached to the release.
7. The release commit is identifiable.

## 11. Required validation report

The release reviewer must record:
- Commit SHA.
- Rust toolchain version.
- Host platform used for validation.
- cargo fmt result.
- cargo check result.
- cargo build result.
- cargo clippy result.
- Unit and integration test results.
- Documentation validation result.
- Dependency/security review result.
- Failed, blocked or waived criteria with justification.
- Final decision: ACCEPTED or REJECTED.

A rejected release must list corrective actions. The version must not be labelled complete until the release gate is satisfied.


---

## V0.1 implementation-specific acceptance supplement (version 1.1)

The following criteria refine the generic requirements above and are mandatory. If a generic example conflicts with this supplement, this supplement governs V0.1.

### AC-020 — Exact crate topology
- [ ] Workspace contains exactly `ese-domain`, `ese-config` and `ese-observability`.
- [ ] All three are real library crates with tested behavior.
- [ ] No generic `errors` crate exists.
- [ ] No empty placeholder crate exists.
- [ ] No binary application is added without an approved use case.

### AC-021 — Toolchain reproducibility
- [ ] `rust-toolchain.toml` pins Rust 1.100.0.
- [ ] Rust edition is 2024.
- [ ] Workspace declares MSRV 1.85.0.
- [ ] Cargo resolver is 3.
- [ ] rustfmt and Clippy components are requested by the toolchain file.
- [ ] CI runs the pinned toolchain, not an unpinned stable channel.

### AC-022 — Minimal dependencies
- [ ] `ese-domain` has no external dependencies.
- [ ] `ese-config` has no external dependencies.
- [ ] `ese-observability` uses only tracing and tracing-subscriber as direct runtime dependencies.
- [ ] tracing-subscriber enables only the fmt and env-filter features unless a documented need is approved.
- [ ] No future crawler, search, database, HTTP or AI dependency is introduced.

### AC-023 — Domain invariants
- [ ] DocumentId and SourceId reject empty and whitespace-only values.
- [ ] Identifier length and allowed-character policy are documented and tested.
- [ ] LanguageCode supports ar, fr, en and und.
- [ ] Unicode values are preserved.
- [ ] EthicalDecision has exactly the documented Allow, Review and Exclude outcomes.
- [ ] PolicyVersion rejects invalid identifiers.
- [ ] No domain type performs I/O or reads environment variables.

### AC-024 — Configuration determinism
- [ ] Environment variables use the ESE_ prefix.
- [ ] Defaults are documented.
- [ ] Malformed explicit values return errors instead of silently falling back.
- [ ] Environment-dependent tests are isolated and do not depend on test execution order.
- [ ] No secret is required or logged in V0.1.

### AC-025 — Observability lifecycle
- [ ] Subscriber initialization has a documented lifecycle.
- [ ] Initialization failure is handled explicitly.
- [ ] Duplicate initialization does not cause an unexplained panic.
- [ ] Default log filtering is documented.
- [ ] Raw user queries, secrets and personal data are not logged.

### AC-026 — Workspace lint policy
- [ ] Workspace forbids unsafe Rust.
- [ ] Clippy is run for all workspace targets.
- [ ] CI treats Clippy warnings as failures.
- [ ] Formatting is checked rather than automatically rewritten in CI.

### AC-027 — Required developer commands
All commands must pass from the repository root:
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace --all-targets`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace --all-targets`
- [ ] `cargo doc --workspace --no-deps`

### AC-028 — License transparency
- [ ] No license is added without an explicit owner decision.
- [ ] README clearly states that no license has been selected.
- [ ] No documentation claims the repository is open source before a license is chosen.

### AC-029 — CI parity
- [ ] GitHub Actions uses Rust 1.100.0.
- [ ] CI executes the same required commands listed in AC-027.
- [ ] CI fails on any failed required command.
- [ ] The workflow runs for pushes and pull requests targeting the default branch.

