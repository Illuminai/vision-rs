use std::os::raw::c_void;
use std::sync::Arc;

use ash::version::DeviceV1_0;
use ash::vk;

use crate::vulkan::Context;

struct MemoryMapPointer(*mut c_void);

unsafe impl Send for MemoryMapPointer {}

unsafe impl Sync for MemoryMapPointer {}

pub struct Buffer {
    context: Arc<Context>,
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
    mapped_pointer: Option<MemoryMapPointer>,
}


impl Buffer {
    fn new(context: Arc<Context>,
           buffer: vk::Buffer,
           memory: vk::DeviceMemory,
           size: vk::DeviceSize) -> Self {
        Self {
            context,
            buffer,
            memory,
            size,
            mapped_pointer: None,
        }
    }

    pub fn create(context: Arc<Context>,
                  size: vk::DeviceSize,
                  usage: vk::BufferUsageFlags,
                  mem_properties: vk::MemoryPropertyFlags) -> Self {
        let device = context.device().vk_device();
        let buffer = {
            let buffer_info = vk::BufferCreateInfo::builder()
                .size(size)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            unsafe {
                device.create_buffer(&buffer_info, None)
                    .expect("Failed to create buffer")
            }
        };

        let mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let memory = {
            let mem_type = context.find_memory_type_index(
                mem_requirements,
                mem_properties,
            );

            let alloc_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(mem_requirements.size)
                .memory_type_index(mem_type);
            unsafe {
                device
                    .allocate_memory(&alloc_info, None)
                    .expect("Failed to allocate memory")
            }
        };

        unsafe {
            device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("Failed to bind buffer memory")
        };

        Buffer::new(context, buffer, memory, mem_requirements.size)
    }

    pub fn cmd_copy(&self, command_buffer: vk::CommandBuffer, src: &Buffer, size: vk::DeviceSize) {
        let region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size,
        };
        let regions = [region];

        unsafe {
            self.context
                .device()
                .vk_device()
                .cmd_copy_buffer(command_buffer, src.buffer, self.buffer, &regions)
        };
    }

    pub fn map_memory(&mut self) -> *mut c_void {
        if let Some(ptr) = &self.mapped_pointer {
            ptr.0
        } else {
            unsafe {
                let ptr = self
                    .context
                    .device()
                    .vk_device()
                    .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())
                    .expect("Failed to map memory");
                self.mapped_pointer = Some(MemoryMapPointer(ptr));
                ptr
            }
        }
    }

    pub fn unmap_memory(&mut self) {
        if self.mapped_pointer.take().is_some() {
            unsafe {
                self.context.device().vk_device().unmap_memory(self.memory);
            }
        }
    }
}


impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.unmap_memory();
            self.context.device().vk_device().destroy_buffer(self.buffer, None);
            self.context.device().vk_device().free_memory(self.memory, None);
        }
    }
}