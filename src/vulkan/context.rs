use crate::vulkan::{Instance, PhysicalDevice, Surface, Device};
use crate::vulkan::debug::ValidationInfo;
use winit::window::Window;
use ash::Entry;

pub struct Context {
    entry: Entry,
    instance: Instance,
    surface: Surface,
    device: Device,
}

impl Context {
    pub fn new(window: &Window, validation_info: ValidationInfo, required_extensions: Vec<*const i8>) -> Self {
        let entry = Entry::new().expect("Failed to create Entry");

        let instance = Instance::new(&entry, &window, validation_info);
        let surface = Surface::new(&entry, &instance, &window);


        let physical_device = PhysicalDevice::optimal_device(&instance, &surface, required_extensions);
        let device = Device::new(&instance, physical_device);

        Self {
            entry,
            instance,
            surface,
            device,
        }
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
}