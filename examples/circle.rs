use lyon::tessellation::FillOptions;
use rgx::core::*;
use rgx::kit::{self, ZDepth};
use rgx_lyon::{LyonPipeline, ShapeBuilder};

mod sandbox;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() -> Result<(), std::io::Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let size = window.inner_size();

    // Setup renderer
    let mut renderer = Renderer::new(&window)?;

    // Setup render pipeline
    let pipeline: LyonPipeline = renderer.pipeline(Blending::default());

    // Setup texture & sampler
    #[rustfmt::skip]
    let texels: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    let buf = Rgba8::align(&texels);

    // Create 4 by 4 texture and sampler.
    let texture = renderer.texture(2, 2);

    // Setup sprite
    let mut builder = ShapeBuilder::default();
    builder
        .fill_circle(
            lyon::math::Point::new(50., 50.),
            25.,
            &[1., 0., 0., 1.],
            &FillOptions::default(),
        )
        .expect("Error tesselating circle");
    let shape = builder.prepare(&renderer);
    let mut textures = renderer.swap_chain(
        size.width as u32,
        size.height as u32,
        PresentMode::default(),
    );

    // Prepare resources
    renderer.submit(&[Op::Fill(&texture, buf)]);

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
                &pipeline,
                kit::ortho(output.width, output.height, Default::default()),
                &mut frame,
            );

            {
                let mut pass = frame.pass(PassOp::Clear(Rgba::TRANSPARENT), &output);

                pass.set_pipeline(&pipeline);
                shape.draw(&mut pass);
            }
            renderer.present(frame);
        }
        _ => {}
    });
}
