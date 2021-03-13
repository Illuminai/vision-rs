use std::ffi::CStr;

use ash::version::InstanceV1_0;
use ash::vk;
use ash::vk::PhysicalDevice as VkPhysicalDevice;

use crate::vulkan::{Instance, Surface};

#[derive(Clone, Copy)]
pub struct QueueFamilyIndices {
    pub graphics_family: u32,
    pub present_family: u32,
}

pub struct PhysicalDevice {
    physical_device: VkPhysicalDevice,
    queue_family_indices: QueueFamilyIndices,
    required_extensions: Vec<*const i8>,
}

impl PhysicalDevice {
    pub fn optimal_device(instance: &Instance, surface: &Surface, required_extensions: Vec<*const i8>) -> Self {
        let mut devices = PhysicalDevice::physical_devices(instance);
        devices.retain(|device| PhysicalDevice::is_device_suitable(instance, surface, device, &required_extensions));

        let optimal_device = devices.into_iter().next();

        match optimal_device {
            None => panic!("Failed to find optimal device"),
            Some(physical_device) => {
                let (graphics_family, present_family) = PhysicalDevice::find_queue_families(instance, surface, &physical_device);
                let queue_family_indices = QueueFamilyIndices {
                    graphics_family: graphics_family.unwrap(),
                    present_family: present_family.unwrap(),
                };
                Self {
                    physical_device,
                    queue_family_indices,
                    required_extensions,
                }
            }
        }
    }

    pub fn vk_physical_device(&self) -> VkPhysicalDevice {
        self.physical_device
    }

    pub fn queue_family_indices(&self) -> QueueFamilyIndices {
        self.queue_family_indices
    }

    pub fn required_extensions(&self) -> &Vec<*const i8> {
        &self.required_extensions
    }

    fn physical_devices(instance: &Instance) -> Vec<VkPhysicalDevice> {
        let physical_devices = unsafe {
            instance.vk_instance().enumerate_physical_devices()
        }.expect("Failed to enumerate physical devices");

        physical_devices
    }

    fn is_device_suitable(instance: &Instance, surface: &Surface, physical_device: &VkPhysicalDevice, required_extensions: &Vec<*const i8>) -> bool {
        let (graphics_family, present_family) = PhysicalDevice::find_queue_families(instance, surface, physical_device);

        PhysicalDevice::check_extension_support(instance, physical_device, required_extensions)
            && graphics_family.is_some()
            && present_family.is_some()
    }

    fn find_queue_families(instance: &Instance, surface: &Surface, physical_device: &VkPhysicalDevice)
                           -> (Option<u32>, Option<u32>) {
        let queue_families = unsafe {
            instance.vk_instance().get_physical_device_queue_family_properties(*physical_device)
        };

        let mut graphics_family = None;
        let mut present_family = None;

        for (index, queue_family) in queue_families.iter().enumerate() {
            let index = index as u32;
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics_family = Some(index);
            }
            let present_support = unsafe {
                surface.vk_surface()
                    .get_physical_device_surface_support(*physical_device, index, *surface.vk_surface_khr())
            };
            if present_support.unwrap() {
                present_family = Some(index);
            }
            if graphics_family.is_some() && present_family.is_some() {
                break;
            }
        }

        (graphics_family, present_family)
    }

    fn check_extension_support(instance: &Instance, physical_device: &VkPhysicalDevice, required_extensions: &Vec<*const i8>) -> bool {
        let available_extensions = unsafe {
            instance.vk_instance().enumerate_device_extension_properties(*physical_device)
        }.expect("Failed to get device extension properties");

        for required_extension in required_extensions {
            unsafe {
                if !available_extensions
                    .iter()
                    .any(|extension| CStr::from_ptr((&extension.extension_name).as_ptr()) == CStr::from_ptr(*required_extension)) {
                    return false;
                }
            }
        }

        true
    }
}