use crate::vulkan::{Instance, PhysicalDevice, Surface, Device};
use crate::vulkan::debug::ValidationInfo;
use ash::Entry;
use std::ffi::CString;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{WindowEvent, Event};
use crate::vulkan::swapchain::Swapchain;
use std::sync::Arc;

mod vulkan;

fn main() {
    println!("Hello, world!");

    // Winit

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Test")
        .build(&event_loop)
        .unwrap();


    // Vulkan Context

    let entry = Entry::new().unwrap();
    let info = ValidationInfo {
        is_enabled: true,
        required_validation_layers: vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()],
    };

    let instance = Instance::new(&entry, &window, info);
    let surface = Surface::new(&entry, &instance, &window);

    let required_extensions = vec![ash::extensions::khr::Swapchain::name().as_ptr()];
    let physical_device = PhysicalDevice::get_optimal_device(&instance, &surface, required_extensions);

    let device = Arc::new(Device::new(&instance, physical_device));

    // Vulkan impl

    let swapchain = Swapchain::new(&instance, device, &surface,
                                   [window.inner_size().width, window.inner_size().height]);

    // Winit Loop

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}