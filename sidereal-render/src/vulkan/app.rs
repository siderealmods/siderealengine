use ash::{vk::{ApplicationInfo, StructureType, make_api_version, API_VERSION_1_3, InstanceCreateInfo}, Entry, Instance};
use sidereal_window::vulkan::VulkanSupport;

use crate::vulkan::error::Error;

pub struct VulkanPlatformSupport {

}

#[cfg(target_os = "windows")]
impl VulkanPlatformSupport {
    pub const VULKAN_EXTENSIONS: Self = Self {};
}

pub struct VulkanApp {
    instance: Option<Instance>,
}

impl VulkanApp {
    fn create_instance(&mut self, mut extensions: Vec<&'static str>) -> Result<(), Error> {
        let entry = match unsafe { Entry::load() } {
            Ok(entry) => entry,
            Err(..) => { return Err(Error::FailedToInitVulkan); }
        };

        let application_info = ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: "s".as_ptr() as _,
            application_version: make_api_version(0, 0, 0, 0),
            p_engine_name: "s".as_ptr() as _,
            engine_version: make_api_version(0, 0, 0, 0),
            api_version: API_VERSION_1_3,
            ..Default::default()
        };

        let instance_info = InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &application_info,
            pp_enabled_extension_names: extensions.as_mut_ptr() as _,
            enabled_extension_count: extensions.len() as u32,
            enabled_layer_count: 0,
            ..Default::default()
        };

        let instance = match unsafe { entry.create_instance(&instance_info, None) } {
            Ok(instance) => instance,
            Err(..) => { return Err(Error::FailedToCreateInstance); }
        };

        self.instance = Some(instance);

        Ok(())
    }

    pub fn init<T: VulkanSupport>(&mut self, window: &T) -> Result<(), Error> {
        self.create_instance(window.get_required_extensions_list())?;

        Ok(())
    }

    pub fn destroy(&mut self) {
        if let Some(instance) = &self.instance {
            unsafe { instance.destroy_instance(None); }
        }
    }

    pub fn new() -> Self {
        Self {
            instance: None,
        }
    }
}
