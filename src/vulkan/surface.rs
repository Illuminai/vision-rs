use ash::extensions::khr::{Surface as VkSurface};
use ash::{vk, Entry};

use crate::vulkan::Instance;

use ash_window;
use winit::window::Window;

pub struct Surface {
    surface: vk::SurfaceKHR,
    surface_loader: VkSurface,
}

impl Surface {
    pub fn new(entry: &Entry, instance: &Instance, window: &Window) -> Self {
        let surface_loader = VkSurface::new(entry, instance.get());
        let surface = unsafe {
            ash_window::create_surface(entry, instance.get(), window, None)
        }.unwrap();

        Self {
            surface,
            surface_loader,
        }
    }

    pub fn get_surface_loader(&self) -> &VkSurface {
        &self.surface_loader
    }

    pub fn get_surface(&self) -> &vk::SurfaceKHR {
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

