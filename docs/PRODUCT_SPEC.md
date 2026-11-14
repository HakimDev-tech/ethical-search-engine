# Product Specification — Ethical Search Engine

- Document ID: ESE-PRODUCT-001
- Document version: 1.0
- Product release: V0.1
- Status: Baseline specification
- Normative language: MUST, MUST NOT, SHOULD and MAY are used in their ordinary requirements sense.

## 1. Product definition

Ethical Search Engine (ESE) is an independent, privacy-conscious, multilingual web search engine. It aims to retrieve useful information while applying a documented, contextual and auditable ethical content policy.

ESE is a general-purpose search engine. Its ethical positioning is not limited to one religion, ideology, subject area or publisher category. The product must distinguish the ethical eligibility of content from its relevance and from the credibility of its source.

The initial supported languages are Arabic (ar), French (fr) and English (en).

## 2. Product vision and value proposition

ESE seeks to provide:
- Relevant results based on the user's query and likely intent.
- Evidence-informed source and document quality signals.
- A transparent, versioned content policy.
- Privacy by data minimization and non-manipulative defaults.
- No paid influence over organic ranking.
- Multilingual discovery across Arabic, French and English.

The product must not claim that its results are universally truthful, exhaustive or free from all harmful material. Search quality and policy accuracy must be measured and reported with their limitations.

## 3. Product principles

### 3.1 Relevance
The engine SHOULD retrieve documents that satisfy the information need expressed by the query. Exact lexical matching is a baseline, not the complete definition of relevance.

### 3.2 Credibility
Source credibility MUST be evaluated using documented evidence and must not be inferred solely from domain popularity, backlinks, age or institutional status.

### 3.3 Information quality
Document quality SHOULD consider verifiability, substantive value, originality, clarity and appropriate sourcing. Length, writing style or page design MUST NOT be treated as sufficient evidence of quality.

### 3.4 Ethical responsibility
Content eligibility MUST be determined by a published, versioned policy. Sensitive subject matter alone MUST NOT imply exclusion.

### 3.5 Privacy
The system MUST minimize collection and retention of personal data. Search queries MUST NOT be logged in raw form by default. Any future retention or analytics feature requires an explicit privacy assessment.

### 3.6 Transparency
The system SHOULD provide a meaningful explanation for ranking and policy decisions without exposing security-sensitive implementation details.

### 3.7 Independence
Organic ranking MUST NOT be affected by payment, sponsorship, affiliate relationships or commercial agreements. Any future sponsored placement, if permitted, must be clearly separated and labelled.

## 4. Ethical policy model

The policy engine has three outcomes:

- ALLOW: the document is eligible to continue through the indexing pipeline.
- REVIEW: the document requires further contextual evaluation and MUST NOT be treated as approved while unresolved.
- EXCLUDE: the document is ineligible under the applicable policy version.

The policy is balanced and contextual. It must distinguish, where applicable:
- Description from promotion or facilitation.
- Educational, scientific, medical, historical or journalistic discussion from exploitative material.
- Criticism of ideas or beliefs from targeted harassment or hateful conduct.
- A page-level finding from a domain-level finding.

A domain-level restriction requires separate evidence and justification. A domain containing one prohibited page must not automatically cause every page on that domain to be excluded.

Every policy decision MUST record:
- Policy identifier and version.
- Decision outcome.
- Stable reason code.
- Evaluation timestamp.
- Classifier identifier and confidence, if automated classification was used.
- Review status and reviewer reference, if human review occurred.

The system MUST support policy updates and re-evaluation of previously classified documents.

## 5. Multilingual requirements

The foundational data model MUST support:
- Language identifiers for Arabic, French and English.
- Unicode text without lossy conversion.
- UTF-8 serialization and storage.
- Language-specific normalization and tokenization interfaces.
- Arabic text handling without assuming left-to-right display.
- Unknown or undetermined language states.

The initial release does not claim complete linguistic analysis. Language detection, tokenization, stemming, diacritic handling, transliteration and cross-language retrieval require explicit later specifications and language-specific tests.

## 6. Product subsystems

The intended long-term system contains:
1. URL discovery and crawler.
2. Fetching and crawl-policy enforcement.
3. HTML extraction and document normalization.
4. Deduplication and canonicalization.
5. Ethical policy engine.
6. Source registry and credibility evaluation.
7. Document quality evaluation.
8. Search index and index lifecycle management.
9. Query parsing and language processing.
10. Candidate retrieval.
11. Ethical Relevance Engine (ERE) ranking.
12. Search API and user interface.
13. Auditing, metrics and observability.

These are logical responsibilities, not a requirement to create one crate or microservice per item.

## 7. V0.1 scope

V0.1 establishes the repository and software foundation only.

In scope:
- Cargo workspace and pinned toolchain.
- Minimal, compiling crate structure.
- Shared domain primitives and invariants.
- Configuration loading and validation.
- Crate-owned typed errors and safe diagnostic boundaries.
- Structured logging initialization.
- Dependency and module-boundary rules.
- Unit and integration test foundations.
- CI checks.
- Documentation and reproducible developer setup.

Out of scope:
- A production crawler or network fetching.
- Tantivy integration or a search index.
- BM25 or any production retrieval algorithm.
- ERE scoring or source-trust propagation.
- Ethical content classification or automated moderation.
- Embedding models, Candle or ONNX Runtime.
- PostgreSQL integration.
- Public search API or frontend.
- Production deployment.

A capability described as future architecture MUST NOT be represented as implemented in V0.1.

## 8. Future release direction

The current planning sequence is:
- V0.1: engineering foundation.
- V0.2: secure crawler foundation.
- V0.3: extraction, normalization and deduplication.
- V0.4: index management.
- V0.5: lexical retrieval and search API.
- V0.6: ERE ranking baseline and evaluation dataset.
- V0.7: ethical policy engine and auditability.
- V0.8: semantic and hybrid retrieval experiments.
- V0.9: hardening, performance and operational readiness.
- V1.0: public release after product, legal, security and quality gates.

This sequence is a planning baseline and may be revised through documented decisions.

## 9. Success definition

V0.1 succeeds only when all mandatory acceptance criteria in `ACCEPTANCE_CRITERIA.md` pass, the implementation matches the architecture, and the repository can be built and tested reproducibly.

Compilation alone is not sufficient.


## 8.1 V0.2 — Secure crawler foundation

V0.2 is the first bounded network-acquisition release. It accepts explicit HTTP/HTTPS seed URLs and retrieves bounded HTML only after URL validation, destination-address validation, DNS pinning, robots evaluation and host politeness checks.

V0.2 does not discover links, crawl recursively, extract HTML, normalize documents, deduplicate, classify ethical content, index or expose a public search API. Fetched bytes are untrusted acquisition output, not approved or searchable documents.

The crawler must deny private/reserved destinations, prevent uncontrolled redirects, enforce bounded streaming and timeouts, and avoid logging URL query strings or response bodies. The normative security and acceptance contract is defined in the V0.2 crawler documents.
