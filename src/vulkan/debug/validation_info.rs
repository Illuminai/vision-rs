use ash::Entry;
use ash::version::EntryV1_0;
use std::ffi::{CStr, CString};

pub struct ValidationInfo {
    pub is_enabled: bool,
    pub required_validation_layers: Vec<CString>,
}

impl ValidationInfo {
    pub fn check_validation_layer_support(&self, entry: &Entry) -> bool {
        if !self.is_enabled {
            return false;
        }

        let layer_properties = entry.enumerate_instance_layer_properties()
            .expect("Failed to enumerate instance layer properties");

        if layer_properties.len() <= 0 {
            return false;
        }

        for required_layer_name in self.required_validation_layers.iter() {
            let mut layer_found = false;

            for layer_property in layer_properties.iter() {
                let test_layer_name = unsafe {
                    CStr::from_ptr((&layer_property.layer_name).as_ptr())
                }.to_str().expect("Failed to convert string");

                let required_layer_str = required_layer_name.to_str().expect("Failed to convert string");
                if required_layer_str == test_layer_name {
                    layer_found = true;
                    break;
                }
            }

            if layer_found == false {
                return false;
            }
        }
        true
    }
}