# Technical Decisions — Ethical Search Engine

- Document ID: ESE-ADR-001
- Document version: 1.0
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
