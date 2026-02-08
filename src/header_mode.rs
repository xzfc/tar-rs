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

    /// A fine-grained configuration.
    Config(HeaderModeConfig),
}

/// Declares the information that should be included when filling a [`Header`]
/// from filesystem metadata.
///
/// ```
/// # use tar::{Builder, HeaderMode, HeaderModeConfig};
/// # let mut writer = Vec::new();
/// let mut ar = Builder::new(writer);
///
/// // Keep timestamps, but normalize everything else.
/// ar.mode(HeaderMode::Config(
///     HeaderModeConfig::deterministic().preserve_mtime(),
/// ));
///
/// // Keep all metadata, but clamp too new timestamps.
/// ar.mode(HeaderMode::Config(
///     HeaderModeConfig::complete().clamp_mtime(1234567890),
/// ));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HeaderModeConfig {
    pub(crate) override_uid: Option<u64>,
    pub(crate) override_gid: Option<u64>,
    pub(crate) mtime_mode: Mtime,
    pub(crate) normalize_mode: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Mtime {
    Keep,
    Set(u64),
    Clamp(u64),
}

impl HeaderModeConfig {
    // Constructors ============================================================

    /// A new [`HeaderModeConfig`] that matches [`HeaderMode::Complete`].
    pub fn complete() -> HeaderModeConfig {
        HeaderModeConfig {
            override_uid: None,
            override_gid: None,
            mtime_mode: Mtime::Keep,
            normalize_mode: false,
        }
    }

    /// A new [`HeaderModeConfig`] that matches [`HeaderMode::Deterministic`].
    pub fn deterministic() -> HeaderModeConfig {
        HeaderModeConfig {
            override_uid: Some(0),
            override_gid: Some(0),
            // We could in theory set the mtime to zero here, but not all tools
            // seem to behave well when ingesting files with a 0 timestamp.
            // For example, rust-lang/cargo#9512 shows that lldb doesn't ingest
            // files with a zero timestamp correctly.
            mtime_mode: Mtime::Set(DETERMINISTIC_TIMESTAMP),
            normalize_mode: true,
        }
    }

    // Uid =====================================================================

    /// Include the [`Header::uid`] from filesystem metadata.
    pub fn preserve_uid(mut self) -> HeaderModeConfig {
        self.override_uid = None;
        self
    }

    /// Override the [`Header::uid`] with the given value.
    pub fn override_uid(mut self, uid: u64) -> HeaderModeConfig {
        self.override_uid = Some(uid);
        self
    }

    // Gid =====================================================================

    /// Include the [`Header::gid`] from filesystem metadata.
    pub fn preserve_gid(mut self) -> HeaderModeConfig {
        self.override_gid = None;
        self
    }

    /// Override the [`Header::gid`] with the given value.
    pub fn override_gid(mut self, gid: u64) -> HeaderModeConfig {
        self.override_gid = Some(gid);
        self
    }

    // Mtime ===================================================================

    /// Include the [`Header::mtime`] from filesystem metadata.
    pub fn preserve_mtime(mut self) -> HeaderModeConfig {
        self.mtime_mode = Mtime::Keep;
        self
    }

    /// Override the [`Header::mtime`] with the given value.
    pub fn override_mtime(mut self, mtime: u64) -> HeaderModeConfig {
        self.mtime_mode = Mtime::Set(mtime);
        self
    }

    /// Clamp the [`Header::mtime`] to be at most the given value.
    ///
    /// In other words, preserve timestamps for older files, but clamp
    /// timestamps for newer files.
    ///
    /// This mimics GNU tar's `--clamp-mtime` option.
    pub fn clamp_mtime(mut self, max_mtime: u64) -> HeaderModeConfig {
        self.mtime_mode = Mtime::Clamp(max_mtime);
        self
    }

    // Mode ====================================================================

    /// Include the [`Header::mode`] from filesystem metadata.
    pub fn preserve_mode(mut self) -> HeaderModeConfig {
        self.normalize_mode = false;
        self
    }

    /// Restrict [`Header::mode`] to be either `0o755` (for directories and
    /// executable files) or `0o644` (otherwise).
    pub fn normalize_mode(mut self) -> HeaderModeConfig {
        self.normalize_mode = true;
        self
    }
}
