use ash::{Entry, vk};
use ash::extensions::khr::Surface as VkSurface;
use ash_window;
use winit::window::Window;

use crate::vulkan::Instance;

pub struct Surface {
    surface: vk::SurfaceKHR,
    surface_loader: VkSurface,
}

impl Surface {
    pub fn new(entry: &Entry, instance: &Instance, window: &Window) -> Self {
        let surface_loader = VkSurface::new(entry, instance.vk_instance());
        let surface = unsafe {
            ash_window::create_surface(entry, instance.vk_instance(), window, None)
        }.unwrap();

        Self {
            surface,
            surface_loader,
        }
    }

    pub fn vk_surface(&self) -> &VkSurface {
        &self.surface_loader
    }

    pub fn vk_surface_khr(&self) -> &vk::SurfaceKHR {
        &self.surface
    }

    pub fn required_extension_names(window: &Window) -> Vec<*const i8> {
        let pointers = ash_window::enumerate_required_extensions(window)
            .unwrap()
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        pointers
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

