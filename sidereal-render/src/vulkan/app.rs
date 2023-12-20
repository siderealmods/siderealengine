use std::{ffi::{CString, c_void, CStr, c_char}, ptr::{null, null_mut}, borrow::Cow};

use ash::{
    vk::{
        ApplicationInfo, StructureType, make_api_version, API_VERSION_1_3,
        InstanceCreateInfo, DebugUtilsMessengerCreateInfoEXT,
        DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerEXT, DebugUtilsMessengerCallbackDataEXT,
        FALSE, Bool32, InstanceCreateInfoBuilder,
    },
    extensions::ext::DebugUtils,
    Entry, Instance
};
use tracing::{info, debug};

use crate::vulkan::{error::VkError, surface::VulkanWindow};

pub struct VulkanApp {
    debug_mode: bool,
    entry: Entry,
    instance: Option<Instance>,
    debug_messenger: Option<DebugUtilsMessengerEXT>,
}

/// a callback for vulkan debug messanger
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    info!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    FALSE
}


impl VulkanApp {
    const REQUIRED_VALIDATION_LAYERS_DEBUG: [&str; 1] = [
        "VK_LAYER_KHRONOS_validation",
    ];

    fn check_availabilities_validation_layers(&mut self, layers: &[&str]) -> Result<bool, VkError> {
        let availables = match self.entry.enumerate_instance_layer_properties() {
            Ok(availables) => availables,
            Err(..) => return Err(VkError::FailedToCreateValidationLayer),
        };

        for layer in layers {
            let mut found = false;

            for available in &availables {
                let layer_name = unsafe { CStr::from_ptr(available.layer_name.as_ptr()) }.to_str().expect("Failed to validate string.");
                if layer_name == *layer {
                    debug!("Required vulkan layer available: \"{}\"", layer_name);
                    found = true;
                }
            }

            if !found {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn create_instance(&mut self, extensions: &'static [*const c_char]) -> Result<(), VkError> {
        let layers_cstr = Self::REQUIRED_VALIDATION_LAYERS_DEBUG.iter().map(|layer| {
            CString::new(*layer).expect("validation layer names are nul")
        }).collect::<Vec<_>>();

        let layers_cstr = layers_cstr.iter().map(|layer| layer.as_ptr()).collect::<Vec<_>>();

        if self.debug_mode {
            match self.check_availabilities_validation_layers(&Self::REQUIRED_VALIDATION_LAYERS_DEBUG) {
                Ok(status) => {
                    if !status {
                        return Err(VkError::ValidationLayersUnavailable);
                    }
                },
                Err(err) => return Err(err),
            }
        }

        let app_name = CString::new("s").unwrap().as_c_str().as_ptr();

        let application_info = ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: app_name,
            application_version: make_api_version(0, 0, 0, 0),
            p_engine_name: app_name,
            engine_version: make_api_version(0, 0, 0, 0),
            api_version: API_VERSION_1_3,
            ..Default::default()
        };

        let extensions_vec = extensions.to_vec();

        let mut instance_info = InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &application_info,
            pp_enabled_extension_names: extensions_vec.as_ptr(),
            enabled_extension_count: extensions_vec.len() as _,
            pp_enabled_layer_names: null(),
            enabled_layer_count: 0,
            p_next: null(),
            ..Default::default()
        };

        // Enabling debug callbacks
        let mut debug_info = DebugUtilsMessengerCreateInfoEXT::default();

        if self.debug_mode {
            instance_info.enabled_layer_count = layers_cstr.len() as _;

            debug_info.s_type = StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
            debug_info.message_severity =
                DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::ERROR;
            debug_info.message_type =
                DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE;

            debug_info.pfn_user_callback = Some(vulkan_debug_callback);
            debug_info.p_user_data = null_mut();

            instance_info.enabled_layer_count = layers_cstr.len() as _;
            instance_info.pp_enabled_layer_names = layers_cstr.as_ptr();
            instance_info.p_next = &debug_info as *const DebugUtilsMessengerCreateInfoEXT as *const u32 as _;
        }

        let instance = match unsafe { self.entry.create_instance(&instance_info, None) } {
            Ok(instance) => instance,
            Err(..) => { return Err(VkError::FailedToCreateInstance); }
        };

        info!("Vulkan instance successfully initialized.");

        self.instance = Some(instance);
        Ok(())
    }

    pub fn init_with_window<T: VulkanWindow>(&mut self, window: &T) -> Result<(), VkError> {
        self.create_instance(window.get_required_extensions_list())?;

        Ok(())
    }

    pub fn destroy(&mut self) {
        if let Some(instance) = &self.instance {
            unsafe { instance.destroy_instance(None); }
        }
    }

    pub fn new(debug_mode: bool) -> Result<Self, VkError> {
        let entry = match unsafe { Entry::load() } {
            Ok(entry) => entry,
            Err(..) => { return Err(VkError::FailedToInitVulkan); }
        };

        Ok(Self {
            debug_mode,
            entry,
            instance: None,
            debug_messenger: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlatformWindow {}

    impl VulkanWindow for TestPlatformWindow {}

    #[tracing_test::traced_test]
    #[test]
    fn init_vulkan() -> Result<(), VkError> {
        let window = TestPlatformWindow {};

        let mut app = VulkanApp::new(true)?;

        app.init_with_window(&window)?;

        Ok(())
    }
}
