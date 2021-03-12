use crate::vulkan::{Instance, PhysicalDevice, Surface};
use ash::{Device as VkDevice};
use ash::vk;
use ash::version::{InstanceV1_0, DeviceV1_0};
use std::collections::HashSet;
use std::ops::Deref;
use ash::vk::Queue;

pub struct Device {
    device: VkDevice,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    present_queue: Queue,
}

impl Device {
    pub fn new(instance: &Instance, physical_device: PhysicalDevice) -> Self {
        let queue_family_indices = physical_device.queue_family_indices();
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
            .enabled_extension_names(physical_device.required_extensions())
            .enabled_features(&device_features);

        let device = unsafe {
            instance.vk_instance()
                .create_device(physical_device.vk_physical_device(), &device_create_info, None)
        }.unwrap();

        let graphics_queue = unsafe {
            device.get_device_queue(queue_family_indices.graphics_family, 0)
        };

        let present_queue = unsafe {
            device.get_device_queue(queue_family_indices.present_family, 0)
        };

        Self {
            device,
            physical_device,
            graphics_queue,
            present_queue,
        }
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn vk_device(&self) -> &VkDevice {
        &self.device
    }

    pub fn graphics_queue(&self) -> &Queue {
        &self.graphics_queue
    }

    pub fn present_queue(&self) -> &Queue {
        &self.present_queue
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.device.destroy_device(None); }
    }
}