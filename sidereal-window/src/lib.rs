#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_family = "wasm")]
pub mod web;

pub trait Window {
    fn new(
        id: String,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        title: String,
    ) -> Result<Self, ()>
    where
        Self: Sized;

    fn create_child_window(&mut self);

    fn create_dialog_window(&mut self);

    fn create_message_dialog(&mut self, message: String, title: String);
}
