use std::ffi::{CStr, CString};

use hal::pso::XrInstanceCreationError;
use openxr_sys::Result as XrResult;
use std::sync::Arc;

pub enum Backend {}

impl hal::xr::XrBackend for Backend {
    type Instance = Instance;

    fn enumerate_extension_properties(
    ) -> Result<Vec<hal::pso::XrExtensionProperty>, hal::UnsupportedBackend> {
        // TODO: Reconsider this
        let entry = openxr::Entry::load().map_err(|e| {
            info!("Failed to load an OpenXR runtime. {:?}", e);
            hal::UnsupportedBackend
        })?;

        let enumerate_extension_properties = entry.fp().enumerate_instance_extension_properties;

        let required_buffer_size = unsafe {
            let mut capacity: u32 = 0;

            // SAFETY: According to OpenXR documentation passing 0 as the capacity
            // and NULL as the pointer to the output buffer will have the required buffer
            // capacity written into the count output. Additionally, for this function,
            // passing NULL as the first argument will have it return properties for all
            // extensions.
            // References:
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#buffer-size-parameters
            enumerate_extension_properties(
                std::ptr::null(),
                0,
                &mut capacity,
                std::ptr::null_mut(),
            );

            capacity
        };

        let extension_properties = unsafe {
            let mut written_count = 0;

            let mut result_buffer = vec![
                openxr_sys::ExtensionProperties::out(std::ptr::null_mut());
                required_buffer_size as usize
            ];

            // SAFETY: Per the OpenXR specification:
            // Parameter 1: A NULL pointer. We are not getting properties for a specific layer.
            // Parameter 2: The capacity of the result buffer.
            // Parameter 3: A pointer to a `u32` which will contain the amount of items written.
            // Parameter 4: The pointer to the result buffer.
            // References:
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
            enumerate_extension_properties(
                std::ptr::null(),
                result_buffer.capacity() as _,
                &mut written_count,
                result_buffer.as_mut_ptr() as _,
            );
            // Truncate the buffer to the amount of results written.
            result_buffer.truncate(written_count as usize);

            result_buffer
        };

        Ok(extension_properties
            .iter()
            .map(|property| {
                // SAFETY: We truncated potentially invalid elements from the `Vec`
                let property = unsafe { property.assume_init() };

                hal::pso::XrExtensionProperty {
                    name: unsafe {
                        CStr::from_ptr(property.extension_name.as_ptr())
                            .to_string_lossy()
                            .into_owned()
                    },
                    version: property.extension_version,
                }
            })
            .collect())
    }

    fn enumerate_layers() -> Result<Vec<hal::pso::XrApiLayerProperties>, hal::UnsupportedBackend> {
        // TODO: Reconsider this
        let entry = openxr::Entry::load().map_err(|e| {
            info!("Failed to load an OpenXR runtime. {:?}", e);
            hal::UnsupportedBackend
        })?;

        let enumerate_api_layer_properties = entry.fp().enumerate_api_layer_properties;

        let required_buffer_size = unsafe {
            let mut size: u32 = 0;
            // SAFETY: According to OpenXR documentation passing 0 as the capacity
            // and NULL as the pointer to the output buffer will have the required
            // buffer capacity written into the count output.
            // References:
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#buffer-size-parameters
            enumerate_api_layer_properties(0, &mut size, std::ptr::null_mut());

            // TODO: Check result

            size
        };

        let api_layer_properties = unsafe {
            let mut result_buffer = vec![
                openxr_sys::ApiLayerProperties::out(std::ptr::null_mut());
                required_buffer_size as usize
            ];

            let mut written_count: u32 = 0;

            // SAFETY: Per the OpenXR specification:
            // Parameter 1: the capacity of the properties array.
            // Parameter 2: is a valid pointer to `written_count`
            // Parameter 3: is a pointer to a correctly configured and sized `Vec`
            // This follows the "Valid Usage" section.
            // References:
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties
            enumerate_api_layer_properties(
                result_buffer.capacity() as u32,
                &mut written_count,
                result_buffer.as_mut_ptr() as _,
            );

            // Truncate the buffer to the amount of results written.
            result_buffer.truncate(written_count as usize);

            // TODO: Check for errors

            result_buffer
        };

        Ok(api_layer_properties
            .iter()
            .map(|input| {
                // SAFETY: This is safe because we truncated the buffer to remove
                // elements that aren't init.
                // References:
                // See above where the buffer is truncated.
                let input = unsafe { input.assume_init() };

                hal::pso::XrApiLayerProperties {
                    // TODO: Safety docs
                    layer_name: unsafe {
                        CStr::from_ptr(input.layer_name.as_ptr())
                            .to_string_lossy()
                            .into_owned()
                    },
                    spec_version: String::from("0"), // TODO
                    layer_version: input.layer_version,
                    // TODO: Safety docs
                    description: unsafe {
                        CStr::from_ptr(input.description.as_ptr())
                            .to_string_lossy()
                            .into_owned()
                    },
                }
            })
            .collect())
    }
}

impl hal::xr::InstanceExtXr<Backend> for super::Instance {
    fn create_xr_instance<S: AsRef<str>>(
        &self,
        application_name: S,
        application_version: u32,
        engine_name: Option<S>,
        engine_version: Option<u32>,
        required_layers: &[&str],
        required_extensions: &[&str],
    ) -> Result<Instance, XrInstanceCreationError> {
        // TODO: This is still a hack
        let entry = openxr::Entry::load().map_err(|e| {
            info!("Failed to load an OpenXR runtime. {:?}", e);
            XrInstanceCreationError::Unsupported
        })?;

        // SAFETY: While creating a struct isn't unsafe, ensuring that it was created
        // correctly is important for future calls.
        // According to the OpenXR documentation, the application info struct is created
        // correctly, here is why:
        // `application_name` is a string that is both non-empty and constrained to the
        // max length as defined by OpenXR.
        // `engine_name` is a string that is constrained to the max length as defined by
        // OpenXR.
        // `application_version` is always correct thanks to the type system (u32).
        // `engine_version` is always correct thanks to the type system (u32).
        // `api_version` is retrieved from `openxr_sys` and follows the OpenXR standard.
        //
        // References:
        // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrApplicationInfo
        let application_info = {
            if let Some(engine_name) = &engine_name {
                // Prevents engine names from being larger than the container in ApplicationInfo
                assert!(
                    engine_name.as_ref().len() <= openxr_sys::MAX_ENGINE_NAME_SIZE,
                    "OpenXR engine names must be {} bytes or less",
                    openxr_sys::MAX_APPLICATION_NAME_SIZE
                );
            };

            // Prevents application names from being larger than the container in ApplicationInfo
            assert!(
                application_name.as_ref().len() <= openxr_sys::MAX_APPLICATION_NAME_SIZE,
                "OpenXR application names must be {} bytes or less",
                openxr_sys::MAX_APPLICATION_NAME_SIZE
            );

            // Prevents application names from being empty
            assert!(
                application_name.as_ref().len() > 0,
                "OpenXR application names must be greater than 0 bytes"
            );

            let mut app_info = openxr_sys::ApplicationInfo {
                application_name: [0; openxr_sys::MAX_APPLICATION_NAME_SIZE],
                engine_name: [0; openxr_sys::MAX_ENGINE_NAME_SIZE],
                application_version,
                engine_version: engine_version.map_or(0, |v| v),
                api_version: openxr_sys::CURRENT_API_VERSION,
            };

            for (app_char, slot) in application_name
                .as_ref()
                .bytes()
                .zip(app_info.application_name.iter_mut())
            {
                *slot = app_char as _;
            }

            app_info.application_name[application_name.as_ref().len()] = 0;

            // Its safe to not do anything if `engine_name` is `None` because the
            // buffer is already initialized to 0
            if let Some(name) = engine_name {
                for (engine_char, slot) in name
                    .as_ref()
                    .bytes()
                    .zip(app_info.application_name.iter_mut())
                {
                    *slot = engine_char as _;
                }

                app_info.application_name[application_name.as_ref().len()] = 0;
            }

            app_info
        };

        // Create NULL-terminated CStrings and collect pointers to them
        // into vectors to be passed to OpenXR.
        let required_layers_cstring = required_layers
            .iter()
            .filter_map(|&name| CString::new(name).ok())
            .collect::<Vec<_>>();

        let required_layer_ptrs = required_layers_cstring
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<_>>();

        let required_exts_cstring = required_extensions
            .iter()
            .filter_map(|&name| CString::new(name).ok())
            .collect::<Vec<_>>();

        let required_ext_ptrs = required_exts_cstring
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<_>>();

        // SAFETY: Once again, while constructing this is not unsafe, it is important
        // to document that this structure is properly created.
        // `ty` is the correct `XrStructureType`.
        // `next` is NULL, which is expected.
        // `create_flags` is empty, which is expected.
        // `application_info` is valid, see above proof.
        // `enabled_api_layer_count` is equal to the number of
        // layers in the array of enabled layer names.
        // `enabled_api_layer_names` is a pointer to an array of
        //  pointers to UTF-8, null-terminated strings.
        // `enabled_extension_count` is equal to the number of
        // extensions in the array of enabled extensions.
        // `enabled_extension_names` is a pointer to an array of
        // pointers to UTF-8, null-terminated strings.
        //
        // References:
        // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstanceCreateInfo
        let create_info = openxr_sys::InstanceCreateInfo {
            ty: openxr_sys::InstanceCreateInfo::TYPE,
            next: std::ptr::null(),
            create_flags: openxr_sys::InstanceCreateFlags::EMPTY,
            application_info,
            enabled_api_layer_count: required_layer_ptrs.len() as _,
            enabled_api_layer_names: required_layer_ptrs.as_ptr(),
            enabled_extension_count: required_ext_ptrs.len() as _,
            enabled_extension_names: required_ext_ptrs.as_ptr(),
        };

        let instance = {
            let mut instance_handle = openxr_sys::Instance::NULL;

            // SAFETY: The parameters passed to `create_instance` are as OpenXR expects them.
            // See above safety proofs for validation.
            // References:
            // https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance
            let call_result =
                unsafe { (entry.fp().create_instance)(&create_info, &mut instance_handle) };

            match call_result {
                XrResult::SUCCESS => Ok(()),
                _ => Err(XrInstanceCreationError::InternalError(call_result)),
            }?;

            instance_handle
        };

        Ok(Instance {
            // TODO: SAFETY DOCS
            raw: unsafe {
                openxr::raw::Instance::load(&entry, instance).map_err(|e| {
                    info!("Error while getting instance function pointers: {:?}", e);
                    XrInstanceCreationError::Unsupported
                })?
            },
            instance,
        })
    }
}

pub struct Instance {
    instance: openxr_sys::Instance,
    raw: openxr::raw::Instance,
}

impl Drop for Instance {
    fn drop(&mut self) {
        let destroy_instance = self.raw.destroy_instance;

        unsafe { destroy_instance(self.instance) };
    }
}
