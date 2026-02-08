#[allow(unused_imports)]
use crate::Header; // for doc comments

/// A deterministic, arbitrary, non-zero timestamp that use used as `mtime`
/// of headers when [`HeaderMode::Deterministic`] is used.
///
/// This value, chosen after careful deliberation, corresponds to _Jul 23, 2006_,
/// which is the date of the first commit for what would become Rust.
pub const DETERMINISTIC_TIMESTAMP: u64 = 1153704088;

/// Declares the information that should be included when filling a [`Header`]
/// from filesystem metadata.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum HeaderMode {
    /// All supported metadata, including mod/access times and ownership will
    /// be included.
    Complete,

    /// Only metadata that is directly relevant to the identity of a file will
    /// be included. In particular, ownership and mod/access times are excluded.
    Deterministic,
}
