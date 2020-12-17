use thiserror::Error;

/// TODO
#[derive(Debug)]
pub struct XrExtensionProperty {
    /// TODO
    pub name: String,
    /// TODO
    pub version: u32,
}
/// TODO
#[derive(Debug)]
pub struct XrApiLayerProperties {
    /// TODO
    pub layer_name: String,
    /// TODO
    pub spec_version: String,
    /// TODO
    pub layer_version: u32,
    /// TODO
    pub description: String,
}

/// TODO
#[derive(Error, Debug)]
pub enum XrInstanceCreationError {
    /// The Application name is too large or empty.
    #[error("application name is invalid, it may be too large if it isn't empty")]
    InvalidApplicationName,
    /// The engine name is too large.
    #[error("engine name is invalid, it is too large")]
    InvalidEngineName,
    /// OpenXR is unsupported.
    #[error("OpenXR is unsupported")]
    Unsupported,
    /// An internal OpenXR error occured during creation.
    ///
    /// Most OpenXR errors indicate a failure that would
    /// be challenging to recover from, in some situtations.  
    /// It is recommened to panic on an `InternalError`.
    #[error("an internal error occured within OpenXR")]
    InternalError(#[from] openxr_sys::Result),
}
