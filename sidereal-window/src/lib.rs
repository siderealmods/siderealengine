#[cfg(target_os = "windows")]
pub mod win32;

pub trait Window {
    fn new() -> Self;
}
