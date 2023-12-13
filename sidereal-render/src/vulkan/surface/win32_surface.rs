use std::os::raw::c_void;

use ash::{
    vk::{
        Win32SurfaceCreateInfoKHR,
        StructureType, SurfaceKHR
    },
    extensions::khr::Win32Surface
};

use crate::vulkan::error::Error;

pub struct Win32VkSurface {
    surface: Win32Surface,
}

impl Win32VkSurface {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        Self {
            surface: Win32Surface::new(entry, instance)
        }
    }

    pub fn create_surface(&mut self, hwnd: *const c_void, hinstance: *const c_void) -> Result<SurfaceKHR, Error> {
        let create_info = Win32SurfaceCreateInfoKHR {
            s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            hwnd,
            hinstance,
            ..Default::default()
        };

        match unsafe { self.surface.create_win32_surface(&create_info, None) } {
            Ok(surface) => {
                return Ok(surface);
            },
            Err(..) => {
                return Err(Error::FailedToCreateKHRSurface);
            }
        }
    }
}
