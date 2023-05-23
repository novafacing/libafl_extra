//! Extra Observers

#[cfg(all(
    feature = "map_hashing_observer",
    any(
        all(feature = "libafl_git", not(feature = "libafl_crates")),
        all(feature = "libafl_crates", not(feature = "libafl_git"))
    )
))]
pub mod maphash;
