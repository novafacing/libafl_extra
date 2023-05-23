//! Extra components for LibAFL

#[cfg(all(feature = "libafl_git", feature = "libafl_crates"))]
compile_error!("The features `libafl_git` and `libafl_crates` are mutually exclusive.");

pub mod observers;
