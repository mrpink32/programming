use ash::extensions::ext;
use ash::prelude::VkResult;
use ash::{vk, Entry};
use std::ffi;

struct VulkanRenderer {
    instance: ash::Instance,
}

impl VulkanRenderer {
    fn new(entry: &ash::Entry) -> Self {
        // let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry point") };
        let instance = Self::create_instance(&entry);
        Self { instance }
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        let app_name = ffi::CString::new("Learn Vulkan").unwrap();
        let engine_name = ffi::CString::new("Learn Vulkan Engine").unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let instance_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

        let instance = unsafe { entry.create_instance(&instance_info, None).unwrap() };

        instance
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut ffi::c_void,
) -> vk::Bool32 {
    let message = ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();

    println!("[Debug][{}][{}] {:?}", severity, ty, message);

    vk::FALSE
}

struct RendererDebug {
    debug_utils: ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl RendererDebug {
    fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        // Result<Self, Error> {
        let debug_utils = ext::DebugUtils::new(entry, instance);

        let messenger_info = vk::DebugUtilsMessengerCreateInfoEXT {
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(vulkan_debug_utils_callback),
            ..Default::default()
        };

        let debug_messenger = unsafe {
            debug_utils
                .create_debug_utils_messenger(&messenger_info, None)
                .unwrap()
        };

        Self {
            debug_utils,
            debug_messenger,
        }

        // Ok(Self {
        //     debug_utils,
        //     debug_messenger,
        // })
    }

    unsafe fn cleanup(&mut self) {
        self.debug_utils
            .destroy_debug_utils_messenger(self.debug_messenger, None);
    }
}

fn main() {
    // let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry point") };

    // let app_info = vk::ApplicationInfo {
    //     api_version: vk::make_api_version(0, 1, 0, 0),
    //     ..Default::default()
    // };

    // let create_info = vk::InstanceCreateInfo {
    //     p_application_info: &app_info,
    //     ..Default::default()
    // };

    // let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };

    let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry point") };

    let renderer = VulkanRenderer::new(&entry);

    // let debug = RendererDebug::new(&entry, &renderer.instance);

    println!("Hello, world!");
    std::process::exit(0);
}
