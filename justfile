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

# Tez tekshiruv — kundalik, commit'dan oldin (~3s warm). Tarmoq talab qilmaydi.
check: fmt-check lint test doc-check

# CI ning AYNAN o'zi — push'dan oldin (~80s warm). To'liq parity.
ci: check example features msrv package audit semver

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
    cargo hack check --feature-powerset --all-targets
    cargo hack test --each-feature

# `--allow-dirty`: `ci` commit'dan OLDIN ishlatiladi, `cargo package` esa toza tree
# talab qiladi. Biz tekshirayotgan narsa — `include` ro'yxati va Cargo.lock sinxroni —
# dirty tree'da ham to'liq tekshiriladi. Toza tree talabi `publish-check` da
# (`cargo publish --dry-run`) va CI'da (toza checkout) baribir qo'yiladi.
# Nashr paketi: `include` ro'yxati va Cargo.lock sinxroni
package:
    cargo package --locked --allow-dirty

# CVE (cargo-audit) va ishlatilmagan dependency (cargo-machete). Tarmoq talab qiladi.
audit:
    @command -v cargo-audit >/dev/null || { echo "kerak: cargo install cargo-audit"; exit 1; }
    @command -v cargo-machete >/dev/null || { echo "kerak: cargo install cargo-machete"; exit 1; }
    cargo audit
    cargo machete --with-metadata

# Tanlangan versiya bump'i o'zgarishlarni qoplaydimi (crates.io baseline bilan)
semver:
    @command -v cargo-semver-checks >/dev/null || { echo "kerak: cargo install cargo-semver-checks"; exit 1; }
    cargo semver-checks check-release

# Versiya bump'i lint'larni yashirmaydi; CHANGELOG yozishdan oldin ishlating.
# Aynan NIMA breaking ekanini ko'rsatadi
semver-detail:
    cargo semver-checks check-release --all-features --release-type patch

# MSRV pollari. Talab: rustup toolchain install 1.85.0 1.94.0
msrv:
    # `--all-targets` emas: criterion (dev-dep, faqat bench) 1.86 talab qiladi,
    # downstream esa dev-dep'larni yuklamaydi — rust-version iste'molchi uchun.
    cargo +1.85.0 check --no-default-features
    cargo +1.85.0 check --features date,id,serde,zeroize,serialize-secrets
    cargo +1.94.0 check --all-targets --all-features

# ==========================================
# RELIZ
# ==========================================

# Diqqat: working tree TOZA bo'lishi kerak, aks holda cargo "uncommitted changes" beradi.
# Publishga tayyorlikni tekshirish (dry-run) — to'liq `ci` dan keyin
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
