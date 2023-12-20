#[derive(Debug)]
pub enum VkError {
    CStringError,
    FailedToCreateInstance,
    FailedToCreateKHRSurface,
    FailedToCreateValidationLayer,
    FailedToInitVulkan,
    ValidationLayersUnavailable,
}
