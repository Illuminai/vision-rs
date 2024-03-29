use std::sync::Arc;

use ash::version::DeviceV1_0;
use ash::vk;
use ash::vk::RenderPass as VkRenderPass;

use crate::vulkan::{Context, Device, Image};
use crate::vulkan::image::ImageParameters;
use crate::vulkan::texture::Texture;

pub struct RenderPass {
    context: Arc<Context>,
    color_attachment: Option<Texture>,
    depth_attachment: Texture,
    render_pass: vk::RenderPass,
}

impl RenderPass {
    pub fn create(context: Arc<Context>,
                  extent: vk::Extent2D,
                  format: vk::Format,
                  depth_format: vk::Format,
                  msaa_samples: vk::SampleCountFlags) -> Self {
        let render_pass = create_render_pass(context.device(), format, depth_format, msaa_samples);

        let color_attachment = match msaa_samples {
            vk::SampleCountFlags::TYPE_1 => None,
            _ => Some(create_color_texture(&context, format, extent, msaa_samples)),
        };

        let depth_attachment = create_depth_texture(&context, depth_format, extent, msaa_samples);

        Self {
            context,
            color_attachment,
            depth_attachment,
            render_pass,
        }
    }

    pub fn color_attachment(&self) -> Option<&Texture> {
        self.color_attachment.as_ref()
    }

    pub fn depth_attachment(&self) -> &Texture {
        &self.depth_attachment
    }

    pub fn vk_render_pass(&self) -> vk::RenderPass {
        self.render_pass
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.context
                .device()
                .vk_device()
                .destroy_render_pass(self.render_pass, None);
        }
    }
}


fn create_render_pass(
    device: &Device,
    format: vk::Format,
    depth_format: vk::Format,
    msaa_samples: vk::SampleCountFlags,
) -> VkRenderPass {
    let final_image_layout = match msaa_samples {
        vk::SampleCountFlags::TYPE_1 => vk::ImageLayout::PRESENT_SRC_KHR,
        _ => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };

    let mut attachment_descriptions = vec![
        vk::AttachmentDescription::builder()
            .format(format)
            .samples(msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(final_image_layout)
            .build(),
        vk::AttachmentDescription::builder()
            .format(depth_format)
            .samples(msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build(),
    ];
    if msaa_samples != vk::SampleCountFlags::TYPE_1 {
        attachment_descriptions.push(
            vk::AttachmentDescription::builder()
                .format(format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::DONT_CARE)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .build(),
        );
    }

    let color_attachment_references = [vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build()];

    let depth_attachment_reference = vk::AttachmentReference::builder()
        .attachment(1)
        .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

    let resolve_attachment_references = [vk::AttachmentReference::builder()
        .attachment(2)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build()];

    let subpass_descriptions = {
        let mut subpass_desc = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_references)
            .depth_stencil_attachment(&depth_attachment_reference);

        if msaa_samples != vk::SampleCountFlags::TYPE_1 {
            subpass_desc = subpass_desc.resolve_attachments(&resolve_attachment_references);
        }[subpass_desc.build()]
    };

    let subpass_dependency = vk::SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags::empty())
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(
            vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        )
        .build();
    let subpass_dependencies = [subpass_dependency];

    let render_pass_info = vk::RenderPassCreateInfo::builder()
        .attachments(&attachment_descriptions)
        .subpasses(&subpass_descriptions)
        .dependencies(&subpass_dependencies);

    unsafe { device.vk_device().create_render_pass(&render_pass_info, None).expect("Failed to create render pass") }
}

fn create_color_texture(
    context: &Arc<Context>,
    format: vk::Format,
    extent: vk::Extent2D,
    msaa_samples: vk::SampleCountFlags,
) -> Texture {
    let image = Image::create(
        Arc::clone(context),
        ImageParameters {
            memory_properties: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            extent,
            sample_count: msaa_samples,
            format,
            usage: vk::ImageUsageFlags::TRANSIENT_ATTACHMENT
                | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            ..Default::default()
        },
    );

    image.transition_image_layout(
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    );

    let view = image.create_view(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR);

    Texture::new(Arc::clone(context), image, view, None)
}

fn create_depth_texture(
    context: &Arc<Context>,
    format: vk::Format,
    extent: vk::Extent2D,
    msaa_samples: vk::SampleCountFlags,
) -> Texture {
    let image = Image::create(
        Arc::clone(context),
        ImageParameters {
            memory_properties: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            extent,
            sample_count: msaa_samples,
            format,
            usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            ..Default::default()
        },
    );

    image.transition_image_layout(
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    );

    let view = image.create_view(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::DEPTH);

    Texture::new(Arc::clone(context), image, view, None)
}