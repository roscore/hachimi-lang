# Contributing

Thanks for helping with 하찌미랭 / Hachimi Lang.

1. Open an issue for substantial language or CLI changes so the behavior can be discussed.
2. For each behavior change, add a failing test first, run it, and record the RED command and expected failure in `TDD_LOG.md`.
3. Implement the smallest useful change, rerun the test, and record the GREEN command and passing result.
4. Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets` before opening a pull request.
5. Keep documentation bilingual when behavior visible to users changes. Do not add copyrighted game assets, lyrics, or logos, or imply affiliation.

Use Rust 2021 and prefer the standard library. Explain user-facing behavior and test evidence in your pull request. The [Code of Conduct](CODE_OF_CONDUCT.md) applies to all project discussions.
