use hal::adapter::PhysicalDevice;
use hal::{Backend, Instance};

use std::ffi::{CStr, CString};
use std::sync::Arc;

trait XrCreateInstance<B: Backend>: Sized {
    fn create(&self, name: &str, version: u32) -> Result<XrInstance, hal::UnsupportedBackend>;
}

impl XrCreateInstance<super::Backend> for super::Instance {
    fn create(&self, name: &str, version: u32) -> Result<XrInstance, hal::UnsupportedBackend> {
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

impl XrInstance {
    fn create_system(
        &self,
        form_factor: openxr::FormFactor,
        view_type: openxr::ViewConfigurationType,
    ) -> Result<XrSystemInstance, hal::UnsupportedBackend> {
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

        Ok(XrSystemInstance {
            xr_raw: self.xr_raw.clone(),
            system,
        })
    }
}

trait XrSystem<B: Backend>: Sized {
    fn requirements(&self) -> openxr::Result<openxr::vulkan::Requirements>;
}

impl XrSystem<super::Backend> for XrSystemInstance {
    fn requirements(&self) -> openxr::Result<openxr::vulkan::Requirements> {
        self.xr_raw
            .graphics_requirements::<openxr::Vulkan>(self.system)
    }
}

pub struct XrRequirements {
    pub min_api_version_supported: openxr::Version,
    pub max_api_version_supported: openxr::Version,
}

pub struct XrSystemInstance {
    xr_raw: Arc<openxr::Instance>,
    system: openxr::SystemId,
}
