# Tolkien

**Adaptive, modular document interpretation and rendering engine.**

Tolkien aims to identify, understand, process, and present documents according to their formats, origins, structures, and characteristics. Its design is intended to support historical and modern document formats through an extensible system of specialized components.

The project is in its initial development stage. The architecture and implementation will evolve as real document variants are explored and supported.

## Current status

The repository currently contains a minimal Rust application foundation. Document recognition, parsing, semantic representation, theming, and rendering are not implemented yet.

## Build and run

Requires a stable Rust toolchain with Cargo.

```sh
cargo build
cargo run
cargo test
```

The initial executable prints the application name and version. This is a starting point for the document-processing pipeline, not a functional document reader yet.

## Documentation

The project documentation is built with [Zensical](https://zensical.org/).

```sh
python -m pip install -r requirements-docs.txt
zensical serve
```

The documentation site can be built locally with `zensical build --clean`. Changes to documentation are built in CI and deployed to GitHub Pages from `main`.

## Development principles

- Adapt processing and presentation to the characteristics detected in each document.
- Keep the core small and make format-specific capabilities extensible.
- Preserve source semantics and provenance wherever possible.
- Make detection evidence and uncertainty inspectable.
- Provide explicit user overrides and safe recovery paths.
- Add support incrementally, validating each document variant end to end.

See [CONTRIBUTING.md](CONTRIBUTING.md) for local checks and commit conventions.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
