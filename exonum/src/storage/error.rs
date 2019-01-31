// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

//! An implementation of `Error` type.

/// The error type for I/O operations with storage.
///
/// These errors result in a panic. Storage errors are fatal as in the case of
/// database issues, the system stops working. Assuming that there are other
/// nodes and secret keys and other crucial data are not stored in the data base,
/// the operation of the system can be resumed from a backup or by rebooting the node.
#[derive(Fail, Debug, Clone)]
#[fail(display = "{}", message)]
pub struct Error {
    message: String,
}

impl Error {
    /// Creates a new storage error with an information message about the reason.
    pub(crate) fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}
