//! API backends.
//!
//! Exonum API is abstract, its custom interlayer allows adding third-party
//! backends, which are modules that implement API according to certain principles.
//! Currently, only the Actix-web backend is available.

pub mod actix;
