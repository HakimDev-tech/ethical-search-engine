# Technical Decisions — Ethical Search Engine

- Document ID: ESE-ADR-001
- Document version: 1.1
- Target release: V0.1
- Status: Initial decision baseline

This document records decisions that constrain implementation. A decision may be superseded only through an explicit update with rationale and impact analysis.

## ADR-001 — Primary language: Rust

**Decision:** Use Rust as the primary implementation language.

**Context:** The engine needs memory safety, predictable resource use, concurrency and strong performance for crawling, parsing and retrieval workloads.

**Alternatives considered:** Go, C++, Java and Python.

**Rationale:** Rust combines compile-time memory-safety guarantees, zero-cost abstractions, a mature Cargo ecosystem and suitable asynchronous and search libraries.

**Consequences:** The team must follow consistent ownership, error-handling and async conventions. The learning curve is accepted rather than hidden.

## ADR-002 — Architecture: modular monolith

**Decision:** Use a modular monolith organized as a Cargo workspace.

**Context:** The product needs enforceable boundaries, but does not yet have scale or operational requirements that justify microservices.

**Alternatives considered:** Single undifferentiated crate, microservices, distributed event-driven system.

**Rationale:** Separate crates provide compile-time boundaries and focused tests while keeping deployment and local development simple.

**Consequences:** Crate dependencies must be reviewed. Services may be extracted later only when operational evidence justifies it.

## ADR-003 — Full-text search: Tantivy

**Decision:** Use Tantivy as the planned lexical retrieval engine.

**Context:** The product needs an inverted index, BM25 retrieval and field-aware search in a Rust-native library.

**Alternatives considered:** PostgreSQL full-text search, Elasticsearch, OpenSearch and a custom inverted index.

**Rationale:** Tantivy is Rust-native and provides core full-text indexing and retrieval capabilities without requiring a separate search service.

**Consequences:** Index schema, analyzers, lifecycle, versioning and rebuild behavior must be explicitly designed. PostgreSQL, if introduced, remains responsible for relational metadata rather than silently becoming a second search authority.

**Status:** Selected for a future release; not a V0.1 dependency.

## ADR-004 — HTTP API: Axum

**Decision:** Use Axum for the planned search API.

**Context:** A future API needs an asynchronous Rust framework that integrates with Tokio.

**Alternatives considered:** Actix Web, Rocket and direct Hyper use.

**Rationale:** Axum offers typed extractors and a straightforward Tokio integration.

**Consequences:** HTTP handlers must remain at the application boundary and must not contain domain or ranking logic.

**Status:** Selected for a future release; not a V0.1 dependency.

## ADR-005 — Async runtime: Tokio

**Decision:** Use Tokio for asynchronous network and application workloads when they are introduced.

**Context:** The future crawler will perform concurrent I/O.

**Rationale:** Tokio is a mature asynchronous runtime with broad ecosystem support.

**Consequences:** CPU-heavy parsing, classification or inference must not block runtime worker threads. Async must not be introduced into pure synchronous domain logic without need.

**Status:** Deferred until an implemented capability requires it.

## ADR-006 — HTML extraction: Scraper

**Decision:** Use Scraper for initial static HTML parsing and CSS-selector-based extraction.

**Context:** The crawler needs structured extraction without automatically operating a full browser.

**Alternatives considered:** Ad hoc string parsing, browser automation and alternative HTML parsers.

**Rationale:** Scraper provides an appropriate lightweight parsing interface for static HTML.

**Consequences:** JavaScript-rendered content is not automatically supported. Browser rendering is outside the initial product scope.

**Status:** Selected for a future extraction release; not a V0.1 dependency.

## ADR-007 — Structured persistence: PostgreSQL

**Decision:** PostgreSQL is the planned relational store for structured metadata, source profiles, policy versions and audit records.

**Context:** These records need transactions, constraints and queryable relationships.

**Rationale:** PostgreSQL is mature and supports robust relational integrity.

**Consequences:** The team must define data ownership and consistency with the future Tantivy index. PostgreSQL is not required by V0.1.

## ADR-008 — Ranking: Ethical Relevance Engine (ERE)

**Decision:** Develop a proprietary, measurable ranking model called ERE.

**Planned dimensions:**
- Textual relevance.
- Source credibility.
- Information quality.
- Freshness.
- Query-intent relevance.

**Context:** Popularity and commercial optimization alone do not represent the product's goals.

**Rationale:** A transparent multi-signal model can be evaluated against a labelled benchmark and improved empirically.

**Consequences:**
- Each signal needs an operational definition.
- Signals need normalization before combination.
- Weights must be calibrated on evaluation data, not chosen solely by intuition.
- The system must compare ERE against a BM25 baseline.
- Ranking explanations must reflect actual contributions.

**Important boundary:** Ethical eligibility is a separate policy decision, not a compensable ranking signal. Privacy is a system-level requirement, not a ranking signal.

**Status:** Concept approved; formula, calibration and benchmark remain future design work.

## ADR-009 — Ethical policy: balanced and contextual

**Decision:** Use a versioned policy engine with ALLOW, REVIEW and EXCLUDE outcomes.

**Context:** Keyword-only filtering creates false positives and cannot reliably distinguish context.

**Rationale:** Educational, medical, scientific, historical and journalistic contexts must be distinguished from promotion, exploitation or facilitation of prohibited material where the policy defines that distinction.

**Consequences:**
- Policy rules must be explicit and versioned.
- Domain-level and document-level decisions must be distinct.
- Ambiguous documents remain in REVIEW until resolved.
- False positives and false negatives must be measured.
- Decisions must be auditable and re-evaluable.

## ADR-010 — Multilingual foundation

**Decision:** Arabic, French and English are foundational product languages.

**Context:** The engine must serve multilingual users and content sources.

**Rationale:** Unicode and language identity must be represented correctly from the beginning.

**Consequences:** Language-specific normalization, tokenization and evaluation are required. Arabic morphology, diacritics and right-to-left presentation cannot be treated as trivial variants of French or English.

## ADR-011 — Semantic retrieval

**Decision:** Defer semantic retrieval until the lexical baseline is measured.

**Alternatives considered:** Candle, ONNX Runtime and hosted embedding APIs.

**Rationale:** Embeddings add model selection, inference, hardware, latency, licensing and evaluation costs. They should solve a demonstrated retrieval gap rather than be included for novelty.

**Consequences:** BM25 must be implemented and benchmarked first. Any semantic model must be evaluated for all three target languages.

## ADR-012 — Observability

**Decision:** Use the Rust tracing ecosystem for structured application diagnostics.

**Context:** Future crawling and indexing pipelines need correlated events and actionable diagnostics.

**Consequences:** Raw search queries, credentials and sensitive user data must not be logged by default. OpenTelemetry may be added when a concrete telemetry backend is selected.

## ADR-013 — Dependency governance

**Decision:** Apply explicit dependency governance.

**Rules:**
- Every direct dependency must have a documented purpose.
- Prefer maintained and appropriately licensed dependencies.
- Review security advisories and license compatibility.
- Pin the Rust toolchain.
- Commit Cargo.lock for the workspace.
- Avoid duplicate functionality and unnecessary transitive dependency growth.
- Do not add model runtimes or network libraries before their features are in scope.

## ADR-014 — Deployment

**Decision:** Keep the system compatible with reproducible containerized deployment.

**Context:** Future deployment should be repeatable across development and production environments.

**Consequences:** Docker and production orchestration are not V0.1 requirements. Deployment topology must be selected after resource and operational requirements are measured.

## ADR-015 — Commercial neutrality

**Decision:** Commercial relationships must not influence organic ranking.

**Consequences:** Future monetization must be isolated from retrieval and ranking. Any sponsored result must be clearly labelled and visually separated from organic results.

## ADR-016 — License and contribution model

**Decision:** No repository license is selected by this document.

**Context:** Choosing a license affects reuse, distribution, contributions and commercial options.

**Consequences:** A license must be explicitly chosen before public distribution of a release. Until then, contributors must not assume that the repository is open source or that its contents may be reused under an implied license.

## Decision change process

To change an accepted decision:
1. Identify the decision and the new evidence.
2. Compare viable alternatives.
3. Describe compatibility, migration, security and maintenance impacts.
4. Update this document and the architecture.
5. Add tests that protect the new boundary.
6. Record the change in Git history.


---

## ADR-017 — V0.1 toolchain and edition

**Decision:** Pin Rust 1.100.0 stable, use Rust edition 2024 and declare MSRV 1.85.0.

**Context:** Reproducible builds require a fixed compiler baseline. Edition 2024 requires Rust 1.85 or later.

**Rationale:** A pinned toolchain avoids silently changing compiler behavior between local development and CI. The declared MSRV provides a compatibility floor for the workspace.

**Consequences:** `rust-toolchain.toml` pins 1.100.0 and requests rustfmt and Clippy. CI uses the same version. Dependency updates must be checked against the declared MSRV; the MSRV may be raised only through a documented decision.

## ADR-018 — Minimal V0.1 crate topology

**Decision:** V0.1 contains exactly three library crates: `ese-domain`, `ese-config` and `ese-observability`.

**Context:** The earlier logical architecture listed more future subsystems than V0.1 needs to compile.

**Rationale:** Three small crates establish real boundaries without empty placeholders or speculative abstractions.

**Consequences:**
- `ese-domain` and `ese-config` use the standard library only.
- `ese-observability` uses `tracing` and `tracing-subscriber` with minimal features.
- No binary application is required in V0.1.
- No generic shared errors crate is created; errors remain owned by the crate that produces them.
- Future crawler, index, ranking, API and storage crates are created only when their release implements real behavior.

## ADR-019 — Configuration source for V0.1

**Decision:** Use typed configuration loaded from documented `ESE_`-prefixed environment variables with explicit defaults. Defer configuration files.

**Context:** V0.1 has no operational need for a file format or configuration-file dependency.

**Rationale:** Standard-library environment parsing keeps the foundation small and avoids adding a parser before a concrete requirement exists.

**Consequences:** Explicit malformed values fail validation. Tests must isolate environment access and avoid nondeterministic concurrent mutation of process-global environment variables.

## ADR-020 — Error ownership

**Decision:** Do not create a generic workspace-wide errors crate.

**Context:** A generic error crate would either become a catch-all or force unrelated modules to depend on abstractions without shared semantics.

**Rationale:** Each crate owns its validation and operational errors. A future application boundary may aggregate errors for reporting.

**Consequences:** Error types stay close to the operation and invariant they represent. Cross-crate error conversion is introduced only when a real application use case requires it.

## ADR-021 — Unsafe Rust policy

**Decision:** Forbid unsafe Rust in all V0.1 workspace crates using the workspace lint policy.

**Context:** No V0.1 requirement needs unsafe code.

**Rationale:** This creates a clear and enforceable safety baseline.

**Consequences:** Any future need for unsafe code requires a separate documented decision, a safety justification and focused review. The baseline must not be weakened merely to silence a compiler error.

## ADR-022 — License remains unresolved

**Decision:** Do not add a LICENSE file until the owner explicitly selects a license.

**Context:** A public GitHub repository is not automatically open source, and the license affects reuse and distribution rights.

**Rationale:** Selecting a license without the owner's intent would be an unjustified legal and product decision.

**Consequences:** The README must state that no license has been selected and that no reuse permission is implied.
