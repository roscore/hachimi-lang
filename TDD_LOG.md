# TDD log

Each RED command was run after writing the named tests and before implementing that behavior. GREEN commands were run after the minimal implementation.

| Cycle | Behavior | RED command and expected result | GREEN command and expected result |
| --- | --- | --- | --- |
| 1 | Token recognition, comments, Unicode positions, nested loops, unmatched loops | `cargo test --test parser` → 5 expected assertion failures (empty parser stub) | `cargo test --test parser` → 5 passed |
| 2 | Byte operations, tape, loops, I/O, runtime safety limits | `cargo test --test vm` → 9 expected assertion failures (VM stub) | `cargo test --test vm` → 9 passed |
| 3 | CLI run/eval/check, UTF-8, limits, errors, help/version | `cargo test --test cli` → 7 expected failures, 1 incidental pass (CLI stub) | `cargo test --test cli` → 8 passed |
| 4 | Shipped greeting example | `cargo test --test example` → expected missing-file failure | `cargo test --test example` → 1 passed |
| 5 | Bare CR and CRLF source positions | `cargo test --lib` → 1 expected position failure (`2:2` instead of `3:2`) | `cargo test --lib` → 1 passed |

Final verification: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`, and `cargo run --quiet -- run examples/hello.hcm` all passed; the CLI printed `Hi!` plus a newline.
