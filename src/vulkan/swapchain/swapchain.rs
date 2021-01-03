use ash::extensions::khr::{Swapchain as VkSwapchain};
use ash::vk;
use crate::vulkan::{Surface, Instance, Device};
use crate::vulkan::swapchain::SwapchainSupportDetails;
use ash::vk::{SwapchainKHR, Image, ImageView};
use ash::version::DeviceV1_0;

use std::sync::Arc;


pub struct Swapchain {
    device: Arc<Device>,

    swapchain_loader: VkSwapchain,
    swapchain: SwapchainKHR,
    images: Vec<Image>,
    image_views: Vec<ImageView>,
}

impl Swapchain {
    pub fn new(instance: &Instance, device: Arc<Device>, surface: &Surface, preferred_dimensions: [u32; 2]) -> Self {
        let support_details = SwapchainSupportDetails::new(device.get_physical_device(), surface);
        let format = support_details.get_optimal_surface_format();
        let present_mode = support_details.get_optimal_present_mode();
        let extent = support_details.get_optimal_extent(preferred_dimensions);
        let image_count = support_details.get_optimal_image_count();

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface.get_surface())
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let queue_family_indices = device.get_physical_device().get_queue_family_indices();
        let queue_families = [queue_family_indices.graphics_family, queue_family_indices.present_family];

        create_info = if queue_family_indices.graphics_family != queue_family_indices.present_family {
            create_info.image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_families)
        } else {
            create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };

        create_info = create_info.pre_transform(support_details.get_capabilities().current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let swapchain_loader = VkSwapchain::new(instance.get(), device.get_device());
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&create_info, None)
                .expect("Failed to create swapchain")
        };

        let images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
                .expect("Failed to get swapchain images")
        };

        let image_views = images.iter()
            .map(|&image| {
                let view_create_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);
                unsafe { device.get_device().create_image_view(&view_create_info, None) }
                    .expect("Failed to create image view")
            }).collect();

        Self {
            device,
            swapchain_loader,
            swapchain,
            images,
            image_views,
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            for image_view in self.image_views.iter() {
                self.device.get_device().destroy_image_view(*image_view, None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }
}