pub trait VulkanSupport {
    fn get_required_extensions_list(&self) -> Vec<&'static str> {
        vec!["VK_KHR_surface"]
    }
}
