# https://just.systems

# CI dagi `RUSTFLAGS: -D warnings` bilan bir xil — lokal va CI bir xil kompilyatsiya
# qilishi uchun. Diqqat: bu cargo fingerprint'ini o'zgartiradi, ya'ni `just test` va
# oddiy `cargo test` orasida almashganda qayta build bo'ladi.
export RUSTFLAGS := "-D warnings"

# Mavjud buyruqlar ro'yxati
default:
    @just --list

# ==========================================
# KUNDALIK ISH
# ==========================================

# Kodni formatlash (fayllarni O'ZGARTIRADI)
fmt:
    cargo fmt --all

# Clippy — barcha target va feature'lar bo'yicha
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Testlar (unit + integration + doctest)
test:
    cargo test --all-features

# Example'ni ishga tushirish
example:
    cargo run --example types_example

# Benchmark (criterion). Talab: Rust 1.86+ (criterion dev-dep)
bench:
    cargo bench

# Hujjatlarni brauzerda ochish
doc:
    cargo doc --no-deps --all-features --open

# ==========================================
# TEKSHIRUVLAR (hech nimani o'zgartirmaydi)
# ==========================================

# Tez tekshiruv — kundalik, commit'dan oldin (~3s warm; cache holatiga bog'liq).
check: fmt-check lint test doc-check

# DB service talab qilmaydigan CI suite — push'dan oldin (~80s warm).
# Jonli PostgreSQL job alohida `postgres-test` orqali ishlaydi.
ci: check example features msrv msrv-sqlx-08 package audit semver

# Format tekshiruvi
fmt-check:
    cargo fmt --all -- --check

# Rustdoc ogohlantirishlari xato sifatida — buzuq havolalarni tutadi
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# powerset: 90 kombinatsiya (~46s) — kompilyatsiya teshiklarini tutadi.
# each-feature: 10 kombinatsiya (~8s) — feature ortidagi doctest/testlarni tutadi.
# Ro'yxat Cargo.toml dan olinadi — qo'lda yozilmaydi, yangi feature avtomatik tushadi.
# Feature kombinatsiyalari bo'yicha check va test
features:
    @command -v cargo-hack >/dev/null || { echo "kerak: cargo install cargo-hack"; exit 1; }
    cargo hack check --feature-powerset --all-targets \
        --group-features sqlx-0_8,sqlx-0_8-postgres \
        --group-features sqlx-0_9,sqlx-0_9-postgres,sqlx,sqlx-postgres
    cargo hack test --each-feature

# `--allow-dirty`: `ci` commit'dan OLDIN ishlatiladi, `cargo package` esa toza tree
# talab qiladi. Biz tekshirayotgan narsa — `include` ro'yxati va Cargo.lock sinxroni —
# dirty tree'da ham to'liq tekshiriladi. Toza tree talabi `publish-check` da
# (`cargo publish --dry-run`) va CI'da (toza checkout) baribir qo'yiladi.
# Nashr paketi: `include` ro'yxati va Cargo.lock sinxroni
package:
    cargo package --locked --allow-dirty

# CVE (cargo-audit) va ishlatilmagan dependency (cargo-machete). Tarmoq talab qiladi.
#
# RUSTSEC-2023-0071 (`rsa`, Marvin Attack) sqlx 0.8 ning IXTIYORIY MySQL drayveridan
# Cargo.lock ga tushadi. `cargo audit` lockfile'ni o'qiydi va feature'larni ko'rmaydi,
# `rsa` esa hech qanday feature yoki target'da build graf'iga kirmaydi (biz faqat
# `postgres` drayverini yoqamiz). Advisory'ni ko'r-ko'rona bosmaslik uchun avval
# reachability tekshiriladi: `rsa` graf'ga qaytsa, recipe yiqiladi.
audit:
    #!/usr/bin/env bash
    set -euo pipefail
    command -v cargo-audit >/dev/null || { echo "kerak: cargo install cargo-audit"; exit 1; }
    command -v cargo-machete >/dev/null || { echo "kerak: cargo install cargo-machete"; exit 1; }
    if cargo tree --target all -i rsa --all-features 2>/dev/null | grep -q '^rsa '; then
        echo "xato: 'rsa' build graf'ida — RUSTSEC-2023-0071 ni qayta baholang" >&2
        exit 1
    fi
    cargo audit --ignore RUSTSEC-2023-0071
    cargo machete --with-metadata

# Tanlangan versiya bump'i o'zgarishlarni qoplaydimi (crates.io baseline bilan)
semver:
    @command -v cargo-semver-checks >/dev/null || { echo "kerak: cargo install cargo-semver-checks"; exit 1; }
    RUSTFLAGS="" cargo semver-checks check-release

# Versiya bump'i lint'larni yashirmaydi; CHANGELOG yozishdan oldin ishlating.
# Aynan NIMA breaking ekanini ko'rsatadi
semver-detail:
    RUSTFLAGS="" cargo semver-checks check-release --all-features --release-type patch

# MSRV pollari. Talab: rustup toolchain install 1.85.0 1.94.0
msrv:
    # `--all-targets` emas: criterion (dev-dep, faqat bench) 1.86 talab qiladi,
    # downstream esa dev-dep'larni yuklamaydi — rust-version iste'molchi uchun.
    cargo +1.85.0 check --no-default-features
    cargo +1.85.0 check --features date,id,serde,zeroize,serialize-secrets
    cargo +1.94.0 check --all-targets --all-features

# sqlx 0.8 liniyasi crate MSRV'ida (1.85) resolve bo'lishini tekshiradi.
#
# Committed Cargo.lock yangi toolchain bilan resolve qilingan, shuning uchun unda
# `icu_*`/`idna_adapter` ning 1.86–1.88 talab qiladigan versiyalari turadi. Rust
# 1.85 iste'molchisining o'z MSRV-aware resolver'i mos versiyalarni tanlaydi —
# shu holatni takrorlash uchun lockfile vaqtincha qayta generatsiya qilinadi va
# recipe tugagach (xato bo'lsa ham) tiklanadi.
msrv-sqlx-08:
    #!/usr/bin/env bash
    set -euo pipefail
    cp Cargo.lock Cargo.lock.msrv-bak
    trap 'mv -f Cargo.lock.msrv-bak Cargo.lock' EXIT
    cargo +1.85.0 generate-lockfile
    cargo +1.85.0 check --features date,id,serde,zeroize,serialize-secrets,sqlx-0_8-postgres

# Jonli PostgreSQL roundtrip, sqlx 0.9. Talab: Rust 1.94 va CREATE DATABASE huquqli
# disposable DATABASE_URL.
postgres-test:
    @test -n "${DATABASE_URL:-}" || { echo "xato: DATABASE_URL kerak (disposable PostgreSQL, CREATE DATABASE huquqi bilan)" >&2; exit 1; }
    cargo +1.94.0 test --all-features --test sqlx_postgres -- --ignored

# Xuddi shu suite sqlx 0.8 kod yo'li ustida. Toolchain 1.94, chunki dev-dependency'lar
# (criterion → 1.86, sqlx 0.9 dev-dep → 1.94) `cargo test` graf'iga har doim kiradi;
# kutubxonaning o'zi 1.85 da resolve bo'lishini `msrv-sqlx-08` isbotlaydi.
postgres-test-08:
    @test -n "${DATABASE_URL:-}" || { echo "xato: DATABASE_URL kerak (disposable PostgreSQL, CREATE DATABASE huquqi bilan)" >&2; exit 1; }
    cargo +1.94.0 test --no-default-features --features date,id,serde,sqlx-0_8-postgres --test sqlx_postgres_0_8 -- --ignored

# ==========================================
# RELIZ
# ==========================================

# Diqqat: working tree TOZA bo'lishi kerak, aks holda cargo "uncommitted changes" beradi.
# Publishga tayyorlikni tekshirish (dry-run) — DB-siz `ci` dan keyin;
# SQLx o'zgargan bo'lsa `postgres-test`ni alohida o'tkazing.
publish-check: ci
    cargo publish --dry-run --locked

# ⚠️ QAYTARILMAYDI: chiqarilgan versiyani qayta yuklab bo'lmaydi, faqat `cargo yank`
# qilinadi va tuzatish uchun yangi versiya chiqarish kerak bo'ladi.
# ⚠️ crates.io ga NASHR QILISH — to'liq tekshiruvdan keyin
publish: publish-check
    cargo publish --locked

# ==========================================
# QO'SHIMCHA
# ==========================================

# Modul strukturasi. Talab: cargo install cargo-modules
tree:
    cargo-modules structure


# ==========================================
# DEV / AI SETUP
# ==========================================

# Codex + Claude + Graphify lokal environment holatini tekshirish
ai-check:
    @echo "== Codex =="
    @command -v codex >/dev/null && codex --version || echo "❌ codex topilmadi"
    @echo
    @echo "== Claude =="
    @command -v claude >/dev/null && claude --version || echo "❌ claude topilmadi"
    @echo
    @echo "== Graphify =="
    @command -v graphify >/dev/null && graphify --version || echo "❌ graphify topilmadi"
    @echo
    @echo "== ripgrep =="
    @command -v rg >/dev/null && rg --version | head -1 || echo "❌ rg topilmadi"
    @echo
    @echo "== Codex MCP =="
    @codex mcp list 2>/dev/null || true
    @echo
    @echo "== Claude MCP =="
    @claude mcp list 2>/dev/null || true

# Graphify skill + MCP'larni joriy kompyuterda sozlash
# Global config'larni faqat shu explicit target o'zgartiradi.
ai-setup:
    @command -v graphify >/dev/null || { echo "❌ graphify kerak"; exit 1; }
    @command -v codex >/dev/null || { echo "❌ codex kerak"; exit 1; }
    @command -v claude >/dev/null || { echo "❌ claude kerak"; exit 1; }

    graphify install --platform codex
    graphify install --platform claude

    @codex mcp remove graphify >/dev/null 2>&1 || true
    codex mcp add graphify -- graphify-mcp graphify-out/graph.json

    @claude mcp remove graphify >/dev/null 2>&1 || true
    claude mcp add --scope user graphify -- graphify-mcp graphify-out/graph.json

    @echo
    @echo "✅ AI environment sozlandi."
    @echo "Tekshirish: just ai-check"
