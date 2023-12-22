#[derive(Debug)]
pub enum VkError {
    CStringError,
    FailedToCreateInstance,
    FailedToCreateKHRSurface,
    FailedToCreateValidationLayer,
    FailedToCreateDebugUtils,
    FailedToInitVulkan,
    InstanceNotInitialized,
    ValidationLayersUnavailable,
}
