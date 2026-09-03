# https://just.systems

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

# Hujjatlarni brauzerda ochish
doc:
    cargo doc --no-deps --all-features --open

# ==========================================
# TEKSHIRUVLAR (hech nimani o'zgartirmaydi)
# ==========================================

# CI dagi tekshiruvlarning aynan o'zi — commit'dan oldin shuni ishlataman.
check: fmt-check lint test audit doc-check

# Format tekshiruvi
fmt-check:
    cargo fmt --all -- --check

# Audit tekshiruv
audit:
    cargo audit && cargo machete --with-metadata


# Rustdoc ogohlantirishlari xato sifatida — buzuq havolalarni tutadi
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Talab: rustup toolchain install 1.85.0
# Cargo.toml dagi rust-version (1.85) bilan build
msrv:
    cargo +1.94.0 check --all-targets --all-features

# ==========================================
# RELIZ
# ==========================================

# Diqqat: working tree TOZA bo'lishi kerak — avval commit qiling,
# aks holda cargo "uncommitted changes" xatosini beradi.
# Publishga tayyorlikni tekshirish (dry-run)
publish-check: check
    cargo publish --dry-run --locked

# ⚠️ QAYTARILMAYDI: chiqarilgan versiyani qayta yuklab bo'lmaydi, faqat
# `cargo yank` qilinadi va tuzatish uchun yangi versiya chiqarish kerak bo'ladi.
# crates.io ga nashr qilish — to'liq tekshiruvdan keyin
publish: publish-check
    cargo publish --locked

# ==========================================
# QO'SHIMCHA
# ==========================================

# Modul strukturasi. Talab: cargo install cargo-modules
tree:
    cargo-modules structure
