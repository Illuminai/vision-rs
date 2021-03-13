use ash::vk;

use crate::vulkan::{PhysicalDevice, Surface};

pub struct SwapchainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn new(physical_device: &PhysicalDevice, surface: &Surface) -> Self {
        let capabilities = unsafe {
            surface.vk_surface()
                .get_physical_device_surface_capabilities(physical_device.vk_physical_device(), *surface.vk_surface_khr())
                .expect("Failed to get physical device surface capabilities")
        };
        let formats = unsafe {
            surface.vk_surface()
                .get_physical_device_surface_formats(physical_device.vk_physical_device(), *surface.vk_surface_khr())
                .expect("Failed to get physical device surface capabilities")
        };
        let present_modes = unsafe {
            surface.vk_surface()
                .get_physical_device_surface_present_modes(physical_device.vk_physical_device(), *surface.vk_surface_khr())
                .expect("Failed to get physical device surface capabilities")
        };

        Self {
            capabilities,
            formats,
            present_modes,
        }
    }

    pub fn optimal_surface_format(&self) -> vk::SurfaceFormatKHR {
        for &format in self.formats.iter() {
            if format.format == vk::Format::B8G8R8A8_SRGB && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                return format;
            }
        }
        self.formats.first().expect("Failed to get optimal surface format").clone()
    }

    pub fn optimal_present_mode(&self) -> vk::PresentModeKHR {
        if self.present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else if self.present_modes.contains(&vk::PresentModeKHR::FIFO) {
            vk::PresentModeKHR::FIFO
        } else {
            vk::PresentModeKHR::IMMEDIATE
        }
    }

    pub fn optimal_extent(&self, preferred_dimensions: [u32; 2]) -> vk::Extent2D {
        if self.capabilities.current_extent.width != std::u32::MAX {
            return self.capabilities.current_extent;
        }
        let min = self.capabilities.min_image_extent;
        let max = self.capabilities.max_image_extent;
        let width = preferred_dimensions[0].min(max.width).max(min.width);
        let height = preferred_dimensions[1].min(max.height).max(min.height);
        vk::Extent2D { width, height }
    }

    pub fn optimal_image_count(&self) -> u32 {
        let max = self.capabilities.max_image_count;
        let mut preferred = self.capabilities.min_image_count + 1;
        if max > 0 && preferred > max {
            preferred = max;
        }
        preferred
    }

    pub fn capabilities(&self) -> &vk::SurfaceCapabilitiesKHR {
        &self.capabilities
    }

    pub fn formats(&self) -> &Vec<vk::SurfaceFormatKHR> {
        &self.formats
    }

    pub fn present_modes(&self) -> &Vec<vk::PresentModeKHR> {
        &self.present_modes
    }
}