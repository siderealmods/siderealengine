use crate::win32::string::IntoPCWSTR;

use windows::Win32::{
    Foundation::{HWND, WPARAM, LPARAM, LRESULT},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        WindowsAndMessaging::*,
        HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForWindow}
    }
};

pub struct Win32VkWindowCreateInfo {
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    title: String,
}

pub struct Win32VkWindow {
    hwnd: HWND,
}

impl Win32VkWindow {
    unsafe extern "system" fn wndproc(hwnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, u_msg, w_param, l_param)
    }

    /// Create window
    fn init_window(
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        title: String
    ) -> Result<HWND, ()> {
        // Enabling HiDPI support
        unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2); }

        let class_name = String::from("SDR").into_pcwstr();
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

        unsafe {RegisterClassW(&wc);}

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name, 
                title.into_pcwstr(),
                WS_OVERLAPPEDWINDOW,
                pos_x,
                pos_y,
                0,
                0,
                None,
                None,
                h_instance,
                None)
        };

        //Adjusting window size to support HiDPI
        let dpi = unsafe{ GetDpiForWindow(hwnd) as f32 };

        unsafe {
            SetWindowPos(hwnd,
                None,
                0,
                0,
                (width as f32 * dpi / 96.0) as i32,
                (height as f32 * dpi / 96.0) as i32,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE
            );
        }

        Ok(hwnd)
    }

    /// Set self as USERDATA to window.
    pub fn set_userdata(&mut self) {
        unsafe { SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, self as *mut Self as _); }
    }

    pub fn new(info: Win32VkWindowCreateInfo) -> Result<Self, ()> {
        let hwnd = Self::init_window(info.pos_x, info.pos_y, info.width, info.height, info.title)?;

        let mut this: Win32VkWindow = Self {
            hwnd
        };

        this.set_userdata();

        Ok(this)
    }

    fn event_loop() {
        
    }

    pub fn run() {
        
    }
}
