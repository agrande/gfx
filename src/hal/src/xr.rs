//! Contains traits and types for gfx-hal's XR support
/// TODO
pub trait XrBackend {
    /// TODO
    type Instance;

    /// TODO
    fn enumerate_extension_properties(
    ) -> Result<Vec<super::pso::XrExtensionProperty>, super::UnsupportedBackend>;
    /// TODO
    #[doc(alias = "xrEnumerateApiLayerProperties")]
    fn enumerate_layers() -> Result<Vec<super::pso::XrApiLayerProperties>, super::UnsupportedBackend>;
}

/// Extends a [`super::Backend`]'s functionality to allow for XR configuration and state querying.
pub trait InstanceExtXr<X: XrBackend>: Sized {
    /// Creates an OpenXR instance.
    ///
    /// Creates an OpenXR instance with the given application name and version. Optionally specifies
    /// an engine name and engine version.
    /// The OpenXR specification comments on the purpose and use of `engine_name` and `engine_version`:
    /// > When implementing a reusable engine that will be used by many applications,
    /// `engine_name` **should** be set to a unique string that identifies the engine,
    /// and `engine_version` **should** encode a representation of the engine's version.
    /// This way, all applications that share this engine version will provide the same
    /// `engine_name` and `engine_version` to the runtime. The engine **should** then enable
    /// individual applications to choose their specific `application_name` and
    /// `application_version`, enabling one application to be distinguished from another
    /// application.
    ///
    /// # Panics
    /// * If `engine_name` or `application_name` exceeds their maximum size respectively.
    /// * If `application_name` is empty.
    ///
    /// # Examples
    /// This example demonstrates usage of OpenXR as a one-off program
    /// where providing an engine name isn't suggested by the specification.
    /// ```
    /// let dummy_instance = this_does_nothing();
    ///
    /// dummy_instance.create_xr_instance(
    ///     "ILD_APPLICATION",
    ///     5,
    ///     None,
    ///     None
    /// );
    /// ```
    ///
    /// This example demostrates usage of the API for a library or engine.
    /// It is suggested to set a unique engine name and version.
    /// ```
    /// let dummy_instance = this_does_nothing();
    ///
    /// dummy_instance.create_xr_instance(
    ///     "ILD_APPLICATION",
    ///     5,
    ///     Some("AWESOME_XR_ENGINE"),
    ///     Some(2)
    ///);
    ///```
    #[doc(alias = "xrCreateInstance")]
    fn create_xr_instance<S: AsRef<str>>(
        &self,
        application_name: S,
        application_version: u32,
        engine_name: Option<S>,
        engine_version: Option<u32>,
        required_layers: &[&str],
        required_extensions: &[&str],
    ) -> Result<X::Instance, super::pso::XrInstanceCreationError>;
}
