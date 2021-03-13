use std::sync::Arc;

use ash::{Entry, vk};
use ash::version::{DeviceV1_0, InstanceV1_0};
use winit::window::Window;

use crate::vulkan::{CommandPool, Device, Instance, PhysicalDevice, Surface};
use crate::vulkan::debug::ValidationInfo;
use crate::vulkan::shared_context::SharedContext;

pub struct Context {
    shared_context: Arc<SharedContext>,
    general_command_pool: CommandPool,
    transient_command_pool: CommandPool,
}

impl Context {
    pub fn new(window: &Window, validation_info: ValidationInfo, required_extensions: Vec<*const i8>) -> Self {
        let shared_context = Arc::new(SharedContext::new(window, validation_info, required_extensions));

        let general_command_pool = CommandPool::new(Arc::clone(&shared_context),
                                                    shared_context.device().physical_device().queue_family_indices(),
                                                    vk::CommandPoolCreateFlags::empty());

        let transient_command_pool = CommandPool::new(Arc::clone(&shared_context),
                                                      shared_context.device().physical_device().queue_family_indices(),
                                                      vk::CommandPoolCreateFlags::TRANSIENT);

        Self {
            shared_context,
            general_command_pool,
            transient_command_pool,
        }
    }

    pub fn new_thread(&self) -> Self {
        let shared_context = Arc::clone(&self.shared_context);

        let general_command_pool = CommandPool::new(Arc::clone(&shared_context),
                                                    shared_context.device().physical_device().queue_family_indices(),
                                                    vk::CommandPoolCreateFlags::empty());

        let transient_command_pool = CommandPool::new(Arc::clone(&shared_context),
                                                      shared_context.device().physical_device().queue_family_indices(),
                                                      vk::CommandPoolCreateFlags::TRANSIENT);

        Self {
            shared_context,
            general_command_pool,
            transient_command_pool,
        }
    }

    pub fn execute_transient<R, F: FnOnce(vk::CommandBuffer) -> R>(&self, executor: F) -> R {
        let command_buffer = {
            let alloc_info = vk::CommandBufferAllocateInfo::builder()
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_pool(self.transient_command_pool.vk_command_pool())
                .command_buffer_count(1);

            unsafe {
                self.device().vk_device()
                    .allocate_command_buffers(&alloc_info)
                    .expect("Failed to allocate command buffer")[0]
            }
        };

        let command_buffers = [command_buffer];

        {
            let begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            unsafe {
                self.device().vk_device()
                    .begin_command_buffer(command_buffer, &begin_info)
                    .expect("Failed to begin command buffer")
            };
        }

        let executor_result = executor(command_buffer);

        unsafe {
            self.device().vk_device()
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer")
        };

        {
            let submit_info = vk::SubmitInfo::builder()
                .command_buffers(&command_buffers)
                .build();
            let submit_infos = [submit_info];
            unsafe {
                let queue = self.device().graphics_queue();
                self.device().vk_device()
                    .queue_submit(queue, &submit_infos, vk::Fence::null())
                    .expect("Failed to submit to queue");
                self.device().vk_device()
                    .queue_wait_idle(queue)
                    .expect("Failed to wait for queue to be idle");
            };
        }

        unsafe {
            self.device().vk_device().free_command_buffers(self.transient_command_pool.vk_command_pool(), &command_buffers);
        };

        executor_result
    }

    pub fn graphics_queue_wait_idle(&self) {
        unsafe {
            self.device().vk_device()
                .queue_wait_idle(self.device().graphics_queue())
                .expect("Failed to wait for queue to be idle")
        }
    }

    pub fn instance(&self) -> &Instance {
        self.shared_context.instance()
    }

    pub fn surface(&self) -> &Surface {
        self.shared_context.surface()
    }

    pub fn device(&self) -> &Device {
        self.shared_context.device()
    }


    pub fn find_memory_type_index(&self, requirements: vk::MemoryRequirements, required_properties: vk::MemoryPropertyFlags) -> u32 {
        let memory_properties = unsafe {
            self.instance().vk_instance().get_physical_device_memory_properties(self.device().physical_device().vk_physical_device())
        };

        for i in 0..memory_properties.memory_type_count {
            if requirements.memory_type_bits & (1 << i) != 0
                && memory_properties.memory_types[i as usize]
                .property_flags
                .contains(required_properties)
            {
                return i;
            }
        }
        panic!("Failed to find suitable memory type.")
    }
}