use std::ffi::c_char;

use crate::{win32::string::ToUTF16String, Window};

use ash::extensions::khr;
use sidereal_render::vulkan::surface::VulkanWindow;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            HiDpi::{
                GetDpiForWindow, SetProcessDpiAwarenessContext,
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            },
            WindowsAndMessaging::*,
        },
    },
};

pub struct Win32WindowCreateInfo {
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    title: String,
}

pub struct Win32Window {
    hwnd: HWND,
}

impl Win32Window {
    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        u_msg: u32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        DefWindowProcW(hwnd, u_msg, w_param, l_param)
    }

    /// Create window
    fn init_window(
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        title: String,
    ) -> Result<HWND, ()> {
        // Enabling HiDPI support
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        }

        let class_name = PCWSTR::from_raw(String::from("s").to_utf16().as_mut_ptr() as *mut _ as _);
        let h_instance = unsafe { GetModuleHandleW(None).unwrap() };

        let wc = unsafe {
            WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                hInstance: h_instance.into(),
                lpszClassName: class_name,
                hCursor: LoadCursorW(None, IDI_APPLICATION).unwrap(),
                ..Default::default()
            }
        };

        unsafe {
            RegisterClassW(&wc);
        }

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name,
                PCWSTR::from_raw(title.to_utf16().as_mut_ptr() as *mut _ as _),
                WS_OVERLAPPEDWINDOW,
                pos_x,
                pos_y,
                0,
                0,
                None,
                None,
                h_instance,
                None,
            )
        };

        //Adjusting window size to support HiDPI
        let dpi = unsafe { GetDpiForWindow(hwnd) as f32 };

        unsafe {
            SetWindowPos(
                hwnd,
                None,
                0,
                0,
                (width as f32 * dpi / 96.0) as i32,
                (height as f32 * dpi / 96.0) as i32,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }

        Ok(hwnd)
    }

    /// Set self as USERDATA to window.
    pub fn set_userdata(&mut self) {
        unsafe {
            SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, self as *mut Self as _);
        }
    }

    pub fn new(info: Win32WindowCreateInfo) -> Result<Self, ()> {
        let hwnd = Self::init_window(info.pos_x, info.pos_y, info.width, info.height, info.title)?;

        let mut this: Win32Window = Self { hwnd };

        this.set_userdata();

        Ok(this)
    }

    fn event_loop() {}

    pub fn run() {}
}

impl Window for Win32Window {
    fn new(
        _id: String,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        title: String,
    ) -> Result<Self, ()> {
        let info = Win32WindowCreateInfo {
            pos_x,
            pos_y,
            width,
            height,
            title,
        };

        Self::new(info)
    }

    fn create_child_window(&mut self) {}

    fn create_dialog_window(&mut self) {}

    fn create_message_dialog(&mut self, message: String, title: String) {}
}

impl VulkanWindow for Win32Window {
    fn get_required_extensions_list(&self) -> &'static [*const c_char] {
        // TODO: Check availability
        const EXTENSIONS: [*const c_char; 2] = [
            khr::Surface::name().as_ptr(),
            khr::Win32Surface::name().as_ptr(),
        ];

        &EXTENSIONS
    }
}
