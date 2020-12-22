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
#[repr(i32)]
#[derive(Debug)]
pub enum XrFormFactor {
    /// The tracked display is attached to the user's head. The user cannot
    /// touch the display itself.
    ///
    /// For example: A VR headset.
    HeadMountedDisplay = 1,
    /// The tracked display is held in the user's hand, independent from the
    /// user's head. The user may be able to touch the display, allowing for
    /// screen-space UI.
    ///
    /// For example: A mobile phone running an AR experience using pass-through
    // video.
    HandheldDisplay = 2,
}

/// TODO
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum XrViewConfigurationType {
    /// One view representing the form factor's one primary display.
    ///
    /// For example: an AR phone's screen.  
    /// This configuration requires one element in XrViewConfigurationProperties
    /// and one projection in each XrCompositionLayerProjection layer.
    PrimaryMono = 1,
    /// Two views representing the form factor's two primary displays, which
    /// map to a left-eye and right-eye view.
    ///
    /// This configuration requires two views in XrViewConfigurationProperties
    /// and two views in each XrCompositionLayerProjection layer.  
    /// View index 0 must represent the left eye and view index 1 must
    /// represent the right eye.
    PrimaryStereo = 2,
    /// TODO
    PrimaryQuadVarjo = 1000037000,
    /// TODO
    SecondaryMonoFirstPersonObserverMSFT = 1000054000,
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

/// TODO
#[repr(i32)]
#[derive(Debug)]
pub enum XrEnvironmentBlendMode {
    /// TODO
    Opaque = 1,
    /// TODO
    Additive = 2,
    /// TODO
    AlphaBlend = 3,
}

impl std::convert::TryFrom<i32> for XrEnvironmentBlendMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Opaque,
            2 => Self::Additive,
            3 => Self::AlphaBlend,
            _ => return Err(()),
        })
    }
}
