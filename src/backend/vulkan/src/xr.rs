use hal::adapter::PhysicalDevice;
use hal::Instance;

use ash::version::InstanceV1_0;
use ash::vk::Handle;

use std::ffi::{CStr, CString};
use std::sync::Arc;

impl super::Instance {
    pub fn create_xr_instance(
        &self,
        name: &str,
        version: u32,
    ) -> Result<XrInstance, hal::UnsupportedBackend> {
        let entry = openxr::Entry::load().map_err(|e| {
            info!("Missing OpenXR entry points: {:?}", e);
            hal::UnsupportedBackend
        })?;

        let app_info = openxr::ApplicationInfo {
            application_name: name,
            application_version: version,
            engine_name: "gfx-rs",
            engine_version: 1,
        };

        let instance_extensions = entry.enumerate_extensions().map_err(|e| {
            info!("Unable to enumerate instance extensions: {:?}", e);
            hal::UnsupportedBackend
        })?;

        let instance_layers = entry.enumerate_layers().map_err(|e| {
            info!("Unable to enumerate instance layers: {:?}", e);
            hal::UnsupportedBackend
        })?;

        let instance = entry
            .create_instance(&app_info, &instance_extensions, &[])
            .map_err(|e| {
                info!("Failed to create OpenXR instance: {:?}", e);
                hal::UnsupportedBackend
            })?;

        if let Ok(properties) = instance.properties() {
            debug!(
                "Loaded OpenXR runtime: {} {}",
                properties.runtime_name, properties.runtime_version
            )
        } else {
            warn!("Unable to get OpenXR instance properties")
        }

        Ok(XrInstance {
            vk_raw: self.raw.clone(),
            // TODO
            vk_instance_exts: self.extensions.clone(),
            xr_raw: Arc::new(instance),
        })
    }
}

pub struct XrInstance {
    vk_raw: Arc<super::RawInstance>,
    vk_instance_exts: Vec<&'static CStr>,
    xr_raw: Arc<openxr::Instance>,
}

impl hal::xr::XrInstance<Backend, super::Backend> for XrInstance {
    fn create_system(
        &self,
        form_factor: openxr::FormFactor,
        view_type: openxr::ViewConfigurationType,
    ) -> Result<XrSystem, hal::UnsupportedBackend> {
        let system = self.xr_raw.system(form_factor).map_err(|e| {
            warn!("Failed to create OpenXR system: {:?}", e);
            hal::UnsupportedBackend
        })?;

        let environment_blend_mode = self
            .xr_raw
            .enumerate_environment_blend_modes(system, view_type)
            .expect("Unable to get blend mode")[0];

        // Check that all the extensions needed to use OpenXR with vulkan are loaded
        let xr_instance_extensions = self
            .xr_raw
            .vulkan_instance_extensions(system)
            .unwrap()
            .split(' ')
            .map(|x| CString::new(x).unwrap())
            .collect::<Vec<_>>();

        for extension in &xr_instance_extensions {
            if !self
                .vk_instance_exts
                .iter()
                .any(|ext| *ext == extension.as_c_str())
            {
                panic!(
                    "OpenXR runtime requires missing Vulkan instance extension {:?}",
                    extension
                );
            }
        }

        Ok(XrSystem {
            xr_raw: self.xr_raw.clone(),
            system,
        })
    }
}

impl hal::xr::XrSystem<Backend, super::Backend> for XrSystem {
    fn requirements(&self) -> openxr::Result<openxr::vulkan::Requirements> {
        self.xr_raw
            .graphics_requirements::<openxr::Vulkan>(self.system)
    }

    fn create_session(
        &self,
        instance: super::Instance,
        physical_device: super::PhysicalDevice,
        device: super::Device,
    ) -> XrSession {
        let session_info = openxr::vulkan::SessionCreateInfo {
            instance: instance.raw.inner.handle().as_raw() as _,
            physical_device: physical_device.handle.as_raw() as _,
            device: device.shared.raw.handle().as_raw() as _,
            queue_family_index: 0,
            queue_index: 0,
        };

        let (session, frame_wait, frame_stream) = unsafe {
            self.xr_raw
                .create_session::<openxr::Vulkan>(self.system, &session_info)
                .unwrap()
        };

        XrSession {
            session,
            frame_stream,
            frame_wait,
        }
    }
}

pub struct XrRequirements {
    pub min_api_version_supported: openxr::Version,
    pub max_api_version_supported: openxr::Version,
}

pub struct XrSystem {
    xr_raw: Arc<openxr::Instance>,
    system: openxr::SystemId,
}

pub struct XrSession {
    session: openxr::Session<openxr::Vulkan>,
    frame_wait: openxr::FrameWaiter,
    frame_stream: openxr::FrameStream<openxr::Vulkan>,
}

impl hal::xr::XrSession<Backend> for XrSession {}

pub enum Backend {}

impl hal::xr::XrBackend for Backend {
    type Backend = super::Backend;
    type Instance = XrInstance;
    type System = XrSystem;
    type Session = XrSession;
}
