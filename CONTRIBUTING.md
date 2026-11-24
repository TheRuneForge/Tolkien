# Contributing

Tolkien is being developed incrementally. Changes should preserve a small core
and be validated against concrete document variants rather than introducing
general abstractions without a demonstrated need.

## Local checks

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --all-targets --all-features
```

Documentation can be previewed locally with:

```sh
python -m pip install -r requirements-docs.txt
zensical serve
```

Build the documentation with `zensical build --clean`.

## Commit messages and semantic versioning

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` for new functionality (SemVer minor).
- `fix:` for bug fixes (SemVer patch).
- `feat!:` or another type with `!` for breaking changes (SemVer major).
- `docs:`, `test:`, `refactor:`, `ci:`, and `chore:` for other changes.

A `BREAKING CHANGE:` footer can also declare a breaking change.

Release Please reads commits merged into `main`, maintains a release pull
request, updates the Rust package version and changelog, and creates a GitHub
release when that release pull request is merged. It does not publish the
project to crates.io.
