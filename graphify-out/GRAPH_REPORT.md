# Graph Report - uz-types  (2026-09-07)

## Corpus Check
- 15 files · ~7,248 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 252 nodes · 581 edges · 10 communities
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 6 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `142c5ebd`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- NumId<Tag, R>
- Self
- BirthDate
- EmailAddress
- Pinfl
- String
- PhoneNumber
- passport.rs
- Q: Bu loyihaning arxitekturasini qisqacha tushuntir

## God Nodes (most connected - your core abstractions)
1. `NumId<Tag, R>` - 40 edges
2. `Id<Tag>` - 36 edges
3. `BirthDate` - 31 edges
4. `IdError` - 19 edges
5. `NumId` - 14 edges
6. `NaiveDate` - 12 edges
7. `Pinfl` - 12 edges
8. `EmailAddress` - 11 edges
9. `u64` - 11 edges
10. `i64` - 11 edges

## Surprising Connections (you probably didn't know these)
- `EmailAddress` --references--> `email domain validation check`  [AMBIGUOUS]
  src/email.rs → graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md
- `EmailAddress` --references--> `email local-part validation check`  [AMBIGUOUS]
  src/email.rs → graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md
- `Explain the architecture of this project (graphify query)` --references--> `EmailAddress`  [EXTRACTED]
  graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md → src/email.rs
- `Explain the architecture of this project (graphify query)` --references--> `TypeError`  [EXTRACTED]
  graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md → src/error.rs
- `trim_in_place()` --references--> `String`  [EXTRACTED]
  src/macros.rs → src/id.rs

## Import Cycles
- 1-file cycle: `src/id.rs -> src/id.rs`

## Hyperedges (group relationships)
- **Domain module aggregation in lib.rs** — src_lib, src_email_emailaddress, src_passport, src_phone_number, src_pinfl, src_birth_date, src_id [EXTRACTED 1.00]
- **Newtype boilerplate helper modules** — src_macros, src_serde_support, src_sqlx_support [INFERRED 0.85]

## Communities (10 total, 0 thin omitted)

### Community 0 - "NumId<Tag, R>"
Cohesion: 0.06
Nodes (31): Clone, Copy, DB, Debug, Decode, Encode, Eq, H (+23 more)

### Community 1 - "Self"
Cohesion: 0.12
Nodes (14): ArgumentBuffer, BoxDynError, IsNull, db_safe_bounds_move_the_error_to_construction(), IdError, numid_conversions_match_the_rest_of_the_crate(), D, Error (+6 more)

### Community 2 - "BirthDate"
Cohesion: 0.11
Nodes (18): AsRef, BirthDate, BirthDateError, DateFormat, NaiveDate, D, Deserialize, Display (+10 more)

### Community 3 - "EmailAddress"
Cohesion: 0.09
Nodes (18): Explain the architecture of this project (graphify query), email domain validation check, EmailAddress, EmailAddressError, email local-part validation check, Result, TypeError, trim_in_place() (+10 more)

### Community 4 - "Pinfl"
Cohesion: 0.16
Nodes (7): Gender, official_examples_pass_strict(), Pinfl, PinflError, Option, Result, Self

### Community 5 - "String"
Cohesion: 0.28
Nodes (14): PhantomData, R, i64, Id, NumId, NumId<Tag, i64>, NumId<Tag, u64>, From (+6 more)

### Community 6 - "PhoneNumber"
Cohesion: 0.19
Nodes (4): PhoneNumber, PhoneNumberError, Result, Self

### Community 7 - "passport.rs"
Cohesion: 0.17
Nodes (3): Passport, PassportError, Result

### Community 8 - "Q: Bu loyihaning arxitekturasini qisqacha tushuntir"
Cohesion: 0.50
Nodes (3): Answer, Q: Bu loyihaning arxitekturasini qisqacha tushuntir, Source Nodes

## Ambiguous Edges - Review These
- `EmailAddress` → `email domain validation check`  [AMBIGUOUS]
  graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md · relation: references
- `EmailAddress` → `email local-part validation check`  [AMBIGUOUS]
  graphify-out/memory/query_20260906_073300_09ebd60e_explain_the_architecture_of_this_project.md · relation: references

## Knowledge Gaps
- **7 isolated node(s):** `Order`, `Answer`, `Source Nodes`, `email domain validation check`, `email local-part validation check` (+2 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 65 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `EmailAddress` and `email domain validation check`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `EmailAddress` and `email local-part validation check`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **Why does `String` connect `String` to `NumId<Tag, R>`, `Self`, `BirthDate`, `EmailAddress`, `PhoneNumber`, `passport.rs`?**
  _High betweenness centrality (0.299) - this node is a cross-community bridge._
- **Why does `BirthDate` connect `BirthDate` to `Pinfl`, `String`?**
  _High betweenness centrality (0.251) - this node is a cross-community bridge._
- **Why does `NumId<Tag, R>` connect `NumId<Tag, R>` to `Self`, `String`?**
  _High betweenness centrality (0.234) - this node is a cross-community bridge._
- **What connects `Order`, `Answer`, `Source Nodes` to the rest of the system?**
  _7 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `NumId<Tag, R>` be split into smaller, more focused modules?**
  _Cohesion score 0.05576441102756892 - nodes in this community are weakly interconnected._