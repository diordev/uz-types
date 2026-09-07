# Graph Report - uz-types  (2026-09-05)

## Corpus Check
- Corpus is ~6,857 words - fits in a single context window. You may not need a graph.

## Summary
- 243 nodes · 560 edges · 10 communities
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Id/NumId trait impl'lari
- BirthDate va sana formatlari
- String newtype'lar va TypeError
- NumIdRepr va BIGINT konversiyasi
- Id<Tag> konstruksiyasi
- Pinfl: checksum va sana
- NumId konversiyalari
- EmailAddress validatsiyasi

## God Nodes (most connected - your core abstractions)
1. `NumId<Tag, R>` - 39 edges
2. `Id<Tag>` - 35 edges
3. `BirthDate` - 31 edges
4. `IdError` - 19 edges
5. `NumId` - 14 edges
6. `NaiveDate` - 12 edges
7. `Pinfl` - 12 edges
8. `u64` - 11 edges
9. `i64` - 11 edges
10. `NumIdRepr` - 11 edges

## Surprising Connections (you probably didn't know these)
- `BirthDate` --references--> `String`  [EXTRACTED]
  src/birth_date.rs → src/id.rs
- `TypeError` --references--> `BirthDateError`  [EXTRACTED]
  src/error.rs → src/birth_date.rs
- `TypeError` --references--> `EmailAddressError`  [EXTRACTED]
  src/error.rs → src/email.rs
- `TypeError` --references--> `IdError`  [EXTRACTED]
  src/error.rs → src/id.rs
- `TypeError` --references--> `PinflError`  [EXTRACTED]
  src/error.rs → src/pinfl.rs

## Import Cycles
- 1-file cycle: `src/id.rs -> src/id.rs`

## Communities (10 total, 0 thin omitted)

### Community 0 - "Id/NumId trait impl'lari"
Cohesion: 0.06
Nodes (31): ArgumentBuffer, BoxDynError, Clone, Copy, DB, Debug, Decode, Encode (+23 more)

### Community 1 - "BirthDate va sana formatlari"
Cohesion: 0.11
Nodes (18): AsRef, BirthDate, BirthDateError, DateFormat, NaiveDate, D, Deserialize, Display (+10 more)

### Community 2 - "String newtype'lar va TypeError"
Cohesion: 0.07
Nodes (11): TypeError, Passport, PassportError, Result, PhoneNumber, PhoneNumberError, Result, Self (+3 more)

### Community 3 - "NumIdRepr va BIGINT konversiyasi"
Cohesion: 0.18
Nodes (9): db_safe_bounds_move_the_error_to_construction(), IdError, D, Error, Ok, Result, S, Self (+1 more)

### Community 4 - "Id<Tag> konstruksiyasi"
Cohesion: 0.10
Nodes (13): Hash, PhantomData, Id, ids_are_typed_and_roundtrip(), numid_conversions_match_the_rest_of_the_crate(), Order, signed_repr_accepts_negative_legacy_ids(), Uuid (+5 more)

### Community 5 - "Pinfl: checksum va sana"
Cohesion: 0.16
Nodes (7): Gender, official_examples_pass_strict(), Pinfl, PinflError, Option, Result, Self

### Community 6 - "NumId konversiyalari"
Cohesion: 0.21
Nodes (12): R, i64, NumId, NumId<Tag, i64>, NumId<Tag, u64>, From, TryFrom, Sealed (+4 more)

### Community 7 - "EmailAddress validatsiyasi"
Cohesion: 0.33
Nodes (3): EmailAddress, EmailAddressError, Result

## Knowledge Gaps
- **1 isolated node(s):** `Order`
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 60 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `NumId konversiyalari` to `Id/NumId trait impl'lari`, `BirthDate va sana formatlari`, `String newtype'lar va TypeError`, `NumIdRepr va BIGINT konversiyasi`, `Id<Tag> konstruksiyasi`?**
  _High betweenness centrality (0.341) - this node is a cross-community bridge._
- **Why does `BirthDate` connect `BirthDate va sana formatlari` to `Pinfl: checksum va sana`, `NumId konversiyalari`?**
  _High betweenness centrality (0.267) - this node is a cross-community bridge._
- **Why does `NumId<Tag, R>` connect `Id/NumId trait impl'lari` to `NumIdRepr va BIGINT konversiyasi`, `Id<Tag> konstruksiyasi`, `NumId konversiyalari`?**
  _High betweenness centrality (0.222) - this node is a cross-community bridge._
- **What connects `Order` to the rest of the system?**
  _1 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Id/NumId trait impl'lari` be split into smaller, more focused modules?**
  _Cohesion score 0.0603921568627451 - nodes in this community are weakly interconnected._
- **Should `BirthDate va sana formatlari` be split into smaller, more focused modules?**
  _Cohesion score 0.11382113821138211 - nodes in this community are weakly interconnected._
- **Should `String newtype'lar va TypeError` be split into smaller, more focused modules?**
  _Cohesion score 0.06756756756756757 - nodes in this community are weakly interconnected._