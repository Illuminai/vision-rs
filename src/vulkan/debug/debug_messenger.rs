use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_void;

use ash::{Entry, Instance};
use ash::extensions::ext::DebugUtils;
use ash::vk;

pub struct DebugMessenger {
    utils_loader: DebugUtils,
    utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugMessenger {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let utils_loader = DebugUtils::new(entry, instance);

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            //.message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING)
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
            .pfn_user_callback(Some(vulkan_debug_callback));

        let utils_messenger = unsafe {
            utils_loader.create_debug_utils_messenger(&create_info, None)
        }.unwrap();

        Self {
            utils_loader,
            utils_messenger,
        }
    }

    pub fn required_extension_names() -> Vec<*const i8> {
        vec![DebugUtils::name().as_ptr()]
    }
}

impl Drop for DebugMessenger {
    fn drop(&mut self) {
        unsafe {
            self.utils_loader.destroy_debug_utils_messenger(self.utils_messenger, None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}