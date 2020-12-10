//! Contains traits and types for gfx-hal's XR support

/// The standard interface for an XR Backend
pub trait XrBackend: Sized {
    /// The graphics backend used with this XrBackend
    type Backend: super::Backend;
    /// TODO
    type Instance: XrInstance<Self, Self::Backend>;
    /// TODO
    type System: XrSystem<Self, Self::Backend>;
    /// TODO
    type Session: XrSession<Self>;
    /// TODO
    type Space: XrSpace<Self>;
}

/// TODO
pub trait XrInstance<X: XrBackend, B: super::Backend> {
    /// TODO
    fn create(instance: &B::Instance, name: &str, version: u32) -> Result<X::Instance, super::UnsupportedBackend>;
    /// TODO
    fn create_system(
        &self,
        form_factor: openxr::FormFactor,
        view_type: openxr::ViewConfigurationType,
    ) -> Result<X::System, super::UnsupportedBackend>;
    /// TODO
    fn poll_event<'buffer>(
        &self,
        event_storage: &'buffer mut openxr::EventDataBuffer,
    ) -> openxr::Result<Option<openxr::Event<'buffer>>>;
}

/// TODO
pub trait XrSystem<X: XrBackend, B: super::Backend>: Sized {
    /// TODO
    fn requirements(&self) -> openxr::Result<openxr::vulkan::Requirements>;
    /// TODO
    fn create_session(
        &self,
        instance: B::Instance,
        physical_device: B::PhysicalDevice,
        device: B::Device,
    ) -> X::Session;
    /// TODO
    fn enumerate_view_configuration_views(
        &self,
        ty: openxr::ViewConfigurationType,
    ) -> openxr::Result<Vec<openxr::ViewConfigurationView>>;
}

/// TODO
pub trait XrSession<X: XrBackend> {
    /// TODO
    fn create_reference_space(
        &self,
        ty: openxr::ReferenceSpaceType,
        pose: openxr::Posef,
    ) -> Result<X::Space, openxr::sys::Result>;
    /// TODO
    fn begin_frame_stream(&mut self);
    /// TODO
    fn end_frame_stream(
        &mut self,
        layers: &[&openxr::CompositionLayerBase<'_, openxr::Vulkan>],
        frame_state: openxr::FrameState,
    );
    /// TODO
    fn create_swapchain(
        &self,
        create_info: &openxr::SwapchainCreateInfo<openxr::Vulkan>,
    ) -> openxr::Result<openxr::Swapchain<openxr::Vulkan>>;
    /// owo
    fn locate_views(
        &self,
        view_configuration_type: openxr::ViewConfigurationType,
        display_time: openxr::Time,
        space: X::Space,
    ) -> openxr::Result<(openxr::ViewStateFlags, Vec<openxr::View>)>;
}

/// TODO
pub trait XrSpace<X: XrBackend> {}

/// TODO
pub trait XrSwapchain<X: XrBackend, B: super::Backend> {
    ///ss
    fn enumerate_images(&self) -> Vec<B::Image>;
    /// ss
    fn acquire_image(&mut self) -> u32;
    /// ss
    fn wait_image(&mut self, timeout: i64);
}
