use std::rc::Rc;

use ash::version::DeviceV1_0;
use ash::vk;
use ash::vk::ShaderModule as VkShaderModule;

use crate::vulkan::Context;

pub struct ShaderModule {
    context: Rc<Context>,
    shader_module: VkShaderModule
}

impl ShaderModule {
    pub fn new(context: Rc<Context>, code: &[u32]) -> Self {
        let create_info = vk::ShaderModuleCreateInfo::builder()
            .code(code);

        let shader_module = unsafe {
            context.device().vk_device().create_shader_module(&create_info, None)
        }.expect("Failed to create shader module");

        Self {
            context,
            shader_module
        }
    }

    pub fn vk_shader_module(&self) -> &VkShaderModule {
        &self.shader_module
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.context.device().vk_device().destroy_shader_module(self.shader_module, None);
        }
    }
}