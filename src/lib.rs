//! `arrow_iter` — Handl stdlib port (`src/index.hnl`).
//!
//! Pure handl: the crate only re-exports the source so a consumer can embed it
//! until the `pkg.hnl` loader can fetch it by name.

pub const ARROW_ITER_HNL: &str = include_str!("index.hnl");
