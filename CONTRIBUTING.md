# Contributing to Rethon

## Development setup

### Prerequisites

- **Rust** (stable toolchain) — install via [rustup](https://rustup.rs/)
- **lefthook** — manages git hooks

```sh
brew install lefthook
```

### Getting started

```sh
git clone <repo>
cd rethon
lefthook install
```

`lefthook install` reads `lefthook.yml` and wires up the pre-commit hook. This only needs to be done once after cloning.

## Pre-commit checks

Every commit runs three checks automatically:

| Check | Command | What it catches |
|-------|---------|-----------------|
| Format | `cargo fmt --check` | Unformatted code |
| Lint | `cargo clippy -- -D warnings` | Common mistakes and style issues |
| Tests | `cargo test` | Regressions |

If any check fails, the commit is blocked. Fix the issue, re-stage, and commit again.

To run the checks manually without committing:

```sh
lefthook run pre-commit
```

To auto-fix formatting before committing:

```sh
cargo fmt
```

## Project structure

```
crates/
  lexer/    — converts source text into a token stream (4-stage pipeline)
  parser/   — converts the token stream into an AST (work in progress)
```

See `crates/lexer/README.md` for a detailed description of the lexer pipeline.
