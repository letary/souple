//! Yoga's generated tests on the souple layout engine — one test binary, one module per Yoga file.
//!
//! `generated/` is written by the importer (`src/bin/import.rs`) and never edited by hand; `support/`
//! is the small harness it writes against. See the README for how to regenerate and run.

// The importer's output is the file of record: `cargo fmt` leaves it as written.
#[rustfmt::skip]
mod generated;
mod support;
