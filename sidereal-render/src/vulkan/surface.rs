use std::ffi::c_char;

use ash::extensions::khr;

#[cfg(target_os = "windows")]
pub(crate) mod win32_surface;

pub trait VulkanWindow {
    fn get_required_extensions_list(&self) -> &'static [*const c_char] {
        const EXTENSIONS: [*const c_char; 1] = [
            khr::Surface::name().as_ptr()
        ];

        &EXTENSIONS
    }
}
