# Graph Report - uz-types  (2026-09-07)

## Corpus Check
- 16 files · ~8,045 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 266 nodes · 608 edges · 10 communities
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6071d9a2`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- NumId<Tag, R>
- Self
- BirthDate
- EmailAddress
- Pinfl
- id.rs
- PhoneNumber
- passport.rs
- Q: Bu loyihaning arxitekturasini qisqacha tushuntir

## God Nodes (most connected - your core abstractions)
1. `NumId<Tag, R>` - 41 edges
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
Nodes (30): Clone, Copy, DB, Debug, Decode, Encode, Eq, H (+22 more)

### Community 1 - "Self"
Cohesion: 0.14
Nodes (13): ArgumentBuffer, BoxDynError, IsNull, db_safe_bounds_move_the_error_to_construction(), IdError, numid_conversions_match_the_rest_of_the_crate(), D, Error (+5 more)

### Community 2 - "BirthDate"
Cohesion: 0.11
Nodes (18): AsRef, BirthDate, BirthDateError, DateFormat, NaiveDate, D, Deserialize, Display (+10 more)

### Community 3 - "EmailAddress"
Cohesion: 0.09
Nodes (18): Explain the architecture of this project (graphify query), email domain validation check, EmailAddress, EmailAddressError, email local-part validation check, Result, TypeError, trim_in_place() (+10 more)

### Community 4 - "Pinfl"
Cohesion: 0.11
Nodes (11): is_leap_year(), is_valid_gregorian_date(), Gender, official_examples_pass_strict(), Pinfl, PinflError, Option, Result (+3 more)

### Community 5 - "id.rs"
Cohesion: 0.21
Nodes (16): PhantomData, R, i64, Id, NumId, NumId<Tag, i64>, NumId<Tag, u64>, Order (+8 more)

### Community 6 - "PhoneNumber"
Cohesion: 0.16
Nodes (7): exact_code_sets_are_classified(), PhoneNumber, PhoneNumberError, Result, Self, unassigned_codes_are_only_structurally_valid(), with_code()

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
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 66 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `EmailAddress` and `email domain validation check`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `EmailAddress` and `email local-part validation check`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **Why does `String` connect `id.rs` to `NumId<Tag, R>`, `Self`, `BirthDate`, `EmailAddress`, `Pinfl`, `PhoneNumber`, `passport.rs`?**
  _High betweenness centrality (0.351) - this node is a cross-community bridge._
- **Why does `BirthDate` connect `BirthDate` to `Pinfl`, `id.rs`?**
  _High betweenness centrality (0.258) - this node is a cross-community bridge._
- **Why does `NumId<Tag, R>` connect `NumId<Tag, R>` to `Self`, `id.rs`?**
  _High betweenness centrality (0.233) - this node is a cross-community bridge._
- **What connects `Order`, `Answer`, `Source Nodes` to the rest of the system?**
  _7 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `NumId<Tag, R>` be split into smaller, more focused modules?**
  _Cohesion score 0.05584415584415584 - nodes in this community are weakly interconnected._