use softbuffer::GraphicsContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        window.set_inner_size(winit::dpi::PhysicalSize {
            width: web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap(),
            height: web_sys::window()
                .unwrap()
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap(),
        });

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&window.canvas())
            .unwrap();
    }

    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                let buffer = (0..((width * height) as usize))
                    .map(|index| {
                        let y = index / (width as usize);
                        let x = index % (width as usize);
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;

                        let color = blue | (green << 8) | (red << 16);

                        color as u32
                    })
                    .collect::<Vec<_>>();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}