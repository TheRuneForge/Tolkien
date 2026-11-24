//! Tolkien: an adaptive, modular document interpretation and rendering engine.
//!
//! The library crate is the foundation for the document-processing system.
//! Functionality will be added incrementally as document variants are supported.

/// The application name.
pub const NAME: &str = "Tolkien";

/// The package version supplied by Cargo at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A short description of the application.
pub const DESCRIPTION: &str =
    "Adaptive, modular document interpretation and rendering engine";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_package_identity() {
        assert_eq!(NAME, "Tolkien");
        assert!(!VERSION.is_empty());
        assert!(DESCRIPTION.contains("document"));
    }
}
