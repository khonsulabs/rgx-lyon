use rgx::core::*;
use rgx::kit::{self};
use rgx_lyon::LyonPipeline;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub trait Sandbox: Sized + 'static {
    fn create(renderer: &Renderer) -> Self;
    fn pipeline(&self) -> &'_ LyonPipeline;
    fn render(&self, pass: &mut Pass);

    fn run() -> Result<(), std::io::Error> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let size = window.inner_size();

        // Setup renderer
        let mut renderer = Renderer::new(&window)?;
        let sandbox = Self::create(&renderer);

        let mut textures = renderer.swap_chain(
            size.width as u32,
            size.height as u32,
            PresentMode::default(),
        );

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    let (w, h) = (size.width as u32, size.height as u32);
                    textures = renderer.swap_chain(w, h, PresentMode::default());
                    *control_flow = ControlFlow::Poll;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    if let VirtualKeyCode::Escape = code {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let output = textures.next();
                let mut frame = renderer.frame();

                renderer.update_pipeline(
                    sandbox.pipeline(),
                    kit::ortho(output.width, output.height, Default::default()),
                    &mut frame,
                );

                {
                    let mut pass = frame.pass(PassOp::Clear(Rgba::TRANSPARENT), &output);

                    sandbox.render(&mut pass);
                }
                renderer.present(frame);
            }
            _ => {}
        });
    }
}
