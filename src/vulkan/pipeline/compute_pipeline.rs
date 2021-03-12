use ash::{vk};
use std::rc::Rc;
use crate::vulkan::Context;
use crate::vulkan::swapchain::Swapchain;
use ash::vk::{Extent3D, SampleCountFlags, ImageTiling, ImageLayout, ImageUsageFlags};
use ash::version::DeviceV1_0;

pub struct ComputePipeline {
    context: Rc<Context>,
    //swapchain: Swapchain,
}

impl ComputePipeline {
    fn new(context: Rc<Context>) -> Self {
        //let swapchain = Swapchain::new(context.clone(), [window.inner_size().width, window.inner_size().height]);
        Self {
            context,
            //swapchain,
        }
    }

    fn create_compute_image(&self) {
        /*let create_info = vk::ImageCreateInfo::builder()
            .format(self.swapchain.format().format)
            .extent(Extent3D {
                width: self.swapchain.extent().width,
                height: self.swapchain.extent().height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::STORAGE | ImageUsageFlags::TRANSFER_SRC);

        let image = unsafe {
            self.context.device().vk_device().create_image(&create_info, None)
        }.expect("Failed to create image");*/

        // TODO: Pipeline
    }

    fn draw(&self) {}
}