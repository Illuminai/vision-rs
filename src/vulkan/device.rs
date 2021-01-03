use crate::vulkan::{Instance, PhysicalDevice, Surface};
use ash::{Device as VkDevice};
use ash::vk;
use ash::version::{InstanceV1_0, DeviceV1_0};
use std::collections::HashSet;
use std::ops::Deref;

pub struct Device {
    device: VkDevice,
    physical_device: PhysicalDevice,
}

impl Device {
    pub fn new(instance: &Instance, physical_device: PhysicalDevice) -> Self {
        let queue_family_indices = physical_device.get_queue_family_indices();
        let queue_priorities = [1.0f32];

        let mut indices = vec![queue_family_indices.graphics_family, queue_family_indices.present_family];
        indices.dedup();

        let queue_create_infos = indices.iter()
            .map(|index| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*index)
                    .queue_priorities(&queue_priorities)
                    .build()
            }).collect::<Vec<_>>();


        let device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(physical_device.get_required_extensions())
            .enabled_features(&device_features);

        let device = unsafe {
            instance.get()
                .create_device(physical_device.get(), &device_create_info, None)
        }.unwrap();

        Self {
            device,
            physical_device,
        }
    }

    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn get_device(&self) -> &VkDevice {
        &self.device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.device.destroy_device(None); }
    }
}