# libafl_extra: Extra components for LibAFL

This crate provides unofficial extra components for LibAFL. These are components that
are either common enough to desire reuse but not common enough to contribute to the
main LibAFL crate or so niche that they would be rejected from the main LibAFL crate.

A few examples:

- An [`Observer`](./src/observers/maphash/mod.rs) that prints the crc32 of a map before
  and after each execution.

<!-- TODO! -->

## Adding a Component

Contributions to this crate are welcome. To make sure this crate is usable in any way
you might want to use LibAFL, each component needs its own feature. For example, the
[`MapHashingObserver`](./src/observers/maphash/mod.rs) has a feature
`"map_hashing_observer"` that gates it. The entire module containing your component
needs to be feature gated, and all dependencies must be optional.

In addition, this crate controls how it depends on LibAFL using a feature selection
(`libafl_git` for the GitHub `main` branch and `libafl_crates` for the crate from
`crates.io`). Your code doesn't have to work with both, but you need to feature gate it
behind some mutually exclusive combination of the two. You can find an example of how to
do that in [`observers/mod.rs`](./src/observers/mod.rs).