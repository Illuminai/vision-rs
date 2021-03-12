use crate::vulkan::{Instance, PhysicalDevice, Surface, Device, Context};
use crate::vulkan::debug::ValidationInfo;
use ash::Entry;
use std::ffi::CString;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{WindowEvent, Event};
use crate::vulkan::swapchain::Swapchain;
use std::sync::Arc;
use std::rc::Rc;

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

    let validation_info = ValidationInfo {
        is_enabled: true,
        required_validation_layers: vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()],
    };

    let required_extensions = vec![ash::extensions::khr::Swapchain::name().as_ptr()];

    let context = Rc::new(Context::new(&window, validation_info, required_extensions));


    // Vulkan impl



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