use std::sync::Arc;

use ash::version::DeviceV1_0;
use ash::vk;

use crate::vulkan::{Context, Device};
use crate::vulkan::physical_device::QueueFamilyIndices;
use crate::vulkan::shared_context::SharedContext;

pub struct CommandPool {
    context: Arc<SharedContext>,
    command_pool: vk::CommandPool,
}

impl CommandPool {
    pub fn new(context: Arc<SharedContext>, queue_families_indices: QueueFamilyIndices, create_flags: vk::CommandPoolCreateFlags) -> Self {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_families_indices.graphics_family)
            .flags(create_flags);
        let command_pool = unsafe {
            context.device().vk_device().create_command_pool(&create_info, None)
                .expect("Failed to create command pool")
        };
        Self {
            context,
            command_pool,
        }
    }

    pub fn vk_command_pool(&self) -> vk::CommandPool {
        self.command_pool
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.context.device().vk_device().destroy_command_pool(self.command_pool, None);
        }
    }
}