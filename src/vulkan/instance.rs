use std::ffi::CString;

use ash::{Entry, Instance as VkInstance, vk};
use ash::version::{EntryV1_0, InstanceV1_0};
use winit::window::Window;

use crate::vulkan::debug::{DebugMessenger, ValidationInfo};
use crate::vulkan::Surface;

pub struct Instance {
    instance: VkInstance,
    debug_messenger: Option<DebugMessenger>,
}

impl Instance {
    pub fn new(entry: &Entry, window: &Window, validation_info: ValidationInfo) -> Self {
        let app_name = CString::new("Vision").unwrap();
        let engine_name = CString::new("Vision Engine").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_version(0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 1, 0));

        let mut extension_names = Instance::required_extensions(window);

        if validation_info.is_enabled && validation_info.required_validation_layers.len() > 0 {
            extension_names.append(&mut DebugMessenger::required_extension_names());
        }

        let mut create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        let layer_name_pointers: Vec<*const i8> = validation_info.required_validation_layers
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        if validation_info.check_validation_layer_support(entry) {
            create_info = create_info.enabled_layer_names(&layer_name_pointers);
        }

        let instance = unsafe { entry.create_instance(&create_info, None) }
            .expect("Failed to create instance!");

        let mut debug_messenger = None;
        if validation_info.is_enabled && validation_info.required_validation_layers.len() > 0 {
            debug_messenger = Some(DebugMessenger::new(entry, &instance));
        }

        Self {
            instance,
            debug_messenger,
        }
    }

    fn required_extensions(window: &Window) -> Vec<*const i8> {
        let mut extensions: Vec<*const i8> = vec![];
        extensions.append(&mut Surface::required_extension_names(window));
        extensions
    }

    pub fn vk_instance(&self) -> &VkInstance {
        &self.instance
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.debug_messenger = None;
        unsafe { self.instance.destroy_instance(None); }
    }
}
