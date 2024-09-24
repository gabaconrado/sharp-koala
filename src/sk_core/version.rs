/// Version information object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkCoreVersion {
    /// The serial number relative to this version
    serial_number: u64,
    /// The functionality array supported by this version, mostly as a helper field since
    /// internally the serial number is what defines the available functionality
    functionalities: Vec<String>,
}

impl SkCoreVersion {
    /// Serial number getter
    pub fn serial_number(&self) -> u64 {
        self.serial_number
    }

    /// Functionalities getter
    pub fn functionalities(&self) -> &[String] {
        &self.functionalities
    }
}

impl SkCoreVersion {
    /// Create a new [`SkCoreVersion`]
    pub fn new<T: ToString>(serial_number: u64, functionalities: Vec<T>) -> Self {
        let functionalities = functionalities.into_iter().map(|e| e.to_string()).collect();
        Self {
            serial_number,
            functionalities,
        }
    }
}

/// All available versions of the Core
///
/// Weakly follows SemVer conventions to enable quick breaking changes identification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkCoreVersions {
    /// Initial version of the core
    ///
    /// Has the following functionalities:
    ///   - version
    V0_0_0,
}

impl SkCoreVersions {
    /// Returns the version object
    pub fn version(&self) -> SkCoreVersion {
        match self {
            SkCoreVersions::V0_0_0 => {
                SkCoreVersion::new(constants::SN_0_0_0, vec![constants::F_VERSION_LABEL])
            }
        }
    }
}

/// Constants for the versions
mod constants {
    pub const SN_0_0_0: u64 = 0;

    pub const F_VERSION_LABEL: &str = "version";
}
