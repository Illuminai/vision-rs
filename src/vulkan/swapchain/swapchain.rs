use std::sync::Arc;

use ash::extensions::khr::Swapchain as VkSwapchain;
use ash::version::DeviceV1_0;
use ash::vk;

use crate::vulkan::{Context, Device, Image, Instance, Surface};
use crate::vulkan::render_pass::RenderPass;
use crate::vulkan::swapchain::SwapchainSupportDetails;

pub struct Swapchain {
    context: Arc<Context>,

    swapchain_loader: VkSwapchain,
    swapchain: vk::SwapchainKHR,
    images: Vec<Image>,
    image_views: Vec<vk::ImageView>,
    framebuffers: Vec<vk::Framebuffer>,

    format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
    image_count: u32,
}

impl Swapchain {
    pub fn new(context: Arc<Context>, render_pass: &RenderPass, preferred_dimensions: [u32; 2]) -> Self {
        let support_details = SwapchainSupportDetails::new(context.device().physical_device(), context.surface());
        let format = support_details.optimal_surface_format();
        let present_mode = support_details.optimal_present_mode();
        let extent = support_details.optimal_extent(preferred_dimensions);
        let image_count = support_details.optimal_image_count();

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*context.surface().vk_surface_khr())
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let queue_family_indices = context.device().physical_device().queue_family_indices();
        let queue_families = [queue_family_indices.graphics_family, queue_family_indices.present_family];

        create_info = if queue_family_indices.graphics_family != queue_family_indices.present_family {
            create_info.image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_families)
        } else {
            create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };

        create_info = create_info.pre_transform(support_details.capabilities().current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let swapchain_loader = VkSwapchain::new(context.instance().vk_instance(), context.device().vk_device());
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&create_info, None)
                .expect("Failed to create swapchain")
        };

        let images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
                .expect("Failed to get swapchain images")
                .iter()
                .map(|image| {
                    Image::create_swapchain_image(context.clone(), *image, format, extent)
                }).collect::<Vec<_>>()
        };

        let image_views: Vec<vk::ImageView> = images.iter()
            .map(|image| {
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
                    .image(image.vk_image());
                unsafe { context.device().vk_device().create_image_view(&view_create_info, None) }
                    .expect("Failed to create image view")
            }).collect();

        let framebuffers: Vec<vk::Framebuffer> = image_views
            .iter()
            .map(|image_view| match render_pass.color_attachment() {
                Some(texture) => vec![texture.view(), render_pass.depth_attachment().view(), *image_view],
                _ => vec![*image_view, render_pass.depth_attachment().view()],
            })
            .map(|attachments| {
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass.vk_render_pass())
                    .attachments(&attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);
                unsafe {
                    context
                        .device()
                        .vk_device()
                        .create_framebuffer(&framebuffer_info, None)
                        .expect("Failed to create framebuffer")
                }
            }).collect();

        Self {
            context,
            swapchain_loader,
            swapchain,
            images,
            image_views,
            format,
            present_mode,
            extent,
            image_count,
            framebuffers
        }
    }

    pub fn format(&self) -> &vk::SurfaceFormatKHR {
        &self.format
    }

    pub fn present_mode(&self) -> &vk::PresentModeKHR {
        &self.present_mode
    }

    pub fn extent(&self) -> &vk::Extent2D {
        &self.extent
    }

    pub fn image_count(&self) -> &u32 {
        &self.image_count
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            for image_view in self.image_views.iter() {
                self.context.device().vk_device().destroy_image_view(*image_view, None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }
}