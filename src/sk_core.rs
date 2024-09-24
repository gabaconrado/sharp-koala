use version::{SkCoreVersion, SkCoreVersions};

/// Version types and definitions
pub(crate) mod version;

/// Current version of the core
const SK_CURRENT_VERSION: SkCoreVersions = SkCoreVersions::V0_0_0;

/// The Sharp Koala core object
///
/// Centralizes all functionality required to run a Sharp Koala API
#[derive(Clone, Debug, Default)]
pub struct SharpKoalaCore;

impl SharpKoalaCore {
    /// Create a new [`SharpKoalaCore`]
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the version of the core
    ///
    /// Each version encodes a unique set of functionalities that is **immutable**
    pub fn version(&self) -> SkCoreVersion {
        SK_CURRENT_VERSION.version()
    }
}
