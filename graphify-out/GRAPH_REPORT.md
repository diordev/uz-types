# Graph Report - uz-types  (2026-09-05)

## Corpus Check
- Corpus is ~6,731 words - fits in a single context window. You may not need a graph.

## Summary
- 244 nodes · 550 edges · 11 communities
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Id<Tag> trait impl'lari
- BirthDate va sana formatlari
- NumIdRepr va BIGINT konversiyasi
- String newtype'lar va TypeError
- Id/NumId konstruksiyasi
- Pinfl: checksum va sana
- EmailAddress validatsiyasi
- Tartiblash va UUID versiyasi
- serde Visitor

## God Nodes (most connected - your core abstractions)
1. `NumId<Tag, R>` - 36 edges
2. `Id<Tag>` - 35 edges
3. `BirthDate` - 31 edges
4. `IdError` - 18 edges
5. `NaiveDate` - 12 edges
6. `NumId` - 12 edges
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

## Communities (11 total, 0 thin omitted)

### Community 0 - "Id<Tag> trait impl'lari"
Cohesion: 0.06
Nodes (30): Clone, Copy, DB, Debug, Decode, Encode, Eq, H (+22 more)

### Community 1 - "BirthDate va sana formatlari"
Cohesion: 0.11
Nodes (18): AsRef, BirthDate, BirthDateError, DateFormat, NaiveDate, D, Deserialize, Display (+10 more)

### Community 2 - "NumIdRepr va BIGINT konversiyasi"
Cohesion: 0.14
Nodes (12): ArgumentBuffer, BoxDynError, IsNull, db_safe_bounds_move_the_error_to_construction(), IdError, D, Error, Ok (+4 more)

### Community 3 - "String newtype'lar va TypeError"
Cohesion: 0.06
Nodes (12): TypeError, Passport, PassportError, Result, PhoneNumber, PhoneNumberError, Result, Self (+4 more)

### Community 4 - "Id/NumId konstruksiyasi"
Cohesion: 0.17
Nodes (14): PhantomData, R, i64, Id, NumId, NumId<Tag, i64>, NumId<Tag, u64>, From (+6 more)

### Community 5 - "Pinfl: checksum va sana"
Cohesion: 0.16
Nodes (7): Gender, official_examples_pass_strict(), Pinfl, PinflError, Option, Result, Self

### Community 6 - "EmailAddress validatsiyasi"
Cohesion: 0.33
Nodes (3): EmailAddress, EmailAddressError, Result

### Community 7 - "Tartiblash va UUID versiyasi"
Cohesion: 0.39
Nodes (3): Ordering, Option, Version

### Community 8 - "serde Visitor"
Cohesion: 0.40
Nodes (5): deserialize_string_newtype(), D, Error, Result, T

## Knowledge Gaps
- **1 isolated node(s):** `Order`
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 60 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `Id/NumId konstruksiyasi` to `Id<Tag> trait impl'lari`, `BirthDate va sana formatlari`, `NumIdRepr va BIGINT konversiyasi`, `String newtype'lar va TypeError`?**
  _High betweenness centrality (0.305) - this node is a cross-community bridge._
- **Why does `BirthDate` connect `BirthDate va sana formatlari` to `Id/NumId konstruksiyasi`, `Pinfl: checksum va sana`?**
  _High betweenness centrality (0.256) - this node is a cross-community bridge._
- **Why does `Id<Tag>` connect `Id<Tag> trait impl'lari` to `NumIdRepr va BIGINT konversiyasi`, `Id/NumId konstruksiyasi`, `Tartiblash va UUID versiyasi`?**
  _High betweenness centrality (0.212) - this node is a cross-community bridge._
- **What connects `Order` to the rest of the system?**
  _1 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Id<Tag> trait impl'lari` be split into smaller, more focused modules?**
  _Cohesion score 0.061224489795918366 - nodes in this community are weakly interconnected._
- **Should `BirthDate va sana formatlari` be split into smaller, more focused modules?**
  _Cohesion score 0.11382113821138211 - nodes in this community are weakly interconnected._
- **Should `NumIdRepr va BIGINT konversiyasi` be split into smaller, more focused modules?**
  _Cohesion score 0.14146341463414633 - nodes in this community are weakly interconnected._