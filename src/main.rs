use std::f32::consts::PI;

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin, Paint,
    Path, Renderer, Solidity,
};
use resource::resource;
use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod engine;
mod helpers;

// Codename "Iosev"
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    helpers::start(1600, 900, "femtovg demo", true);
    #[cfg(target_arch = "wasm32")]
    helpers::start();
}

#[cfg(not(target_arch = "wasm32"))]
use glutin::PossiblyCurrent;

#[cfg(target_arch = "wasm32")]
use winit::window::Window;

pub fn quantize(a: f32, d: f32) -> f32 {
    (a / d + 0.5).trunc() * d
}

pub struct Fonts {
    regular: FontId,
    ext: FontId,
    square: FontId,
}

fn run(
    mut canvas: Canvas<OpenGl>,
    el: EventLoop<()>,
    #[cfg(not(target_arch = "wasm32"))] windowed_context: glutin::WindowedContext<PossiblyCurrent>,
    #[cfg(target_arch = "wasm32")] window: Window,
) {
    let fonts = Fonts {
        regular: canvas
            .add_font_mem(&resource!("assets/fonts/iosevka-term-curly-regular.ttf"))
            .expect("Cannot add font"),
        ext: canvas
            .add_font_mem(&resource!("assets/fonts/iosevka-term-curly-extended.ttf"))
            .expect("Cannot add font"),
        square: canvas
            .add_font_mem(&resource!("assets/fonts/iosevka-square-regular.ttf"))
            .expect("Cannot add font"),
    };

    let mut mousex = 0.0;
    let mut mousey = 0.0;
    let mut dragging = false;
    let mut scale = 1.0;

    let mut engine = engine::Engine::new();

    el.run(move |event, _, control_flow| {
        #[cfg(not(target_arch = "wasm32"))]
        let window = windowed_context.window();

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { ref event, .. } => match event {
                #[cfg(not(target_arch = "wasm32"))]
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CursorMoved {
                    device_id: _, position, ..
                } => {
                    if dragging {
                        let p0 = canvas.transform().inversed().transform_point(mousex, mousey);
                        let p1 = canvas
                            .transform()
                            .inversed()
                            .transform_point(position.x as f32, position.y as f32);

                        canvas.translate(p1.0 - p0.0, p1.1 - p0.1);
                    }

                    mousex = position.x as f32;
                    mousey = position.y as f32;
                }
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta: winit::event::MouseScrollDelta::LineDelta(_, y),
                    ..
                } => {
                    let pt = canvas.transform().inversed().transform_point(mousex, mousey);
                    canvas.translate(pt.0, pt.1);
                    scale *= 1.0 + (y / 10.0);
                    canvas.scale(1.0 + (y / 10.0), 1.0 + (y / 10.0));
                    canvas.translate(-pt.0, -pt.1);
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => match state {
                    ElementState::Pressed => dragging = true,
                    ElementState::Released => dragging = false,
                },
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    engine.on_keyboard_input(input);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.1, 0.1, 0.1));

                let height = size.height as f32;
                let width = size.width as f32;

                let pt = canvas.transform().inversed().transform_point(mousex, mousey);
                let rel_mousex = pt.0;
                let rel_mousey = pt.1;

                engine.draw(&mut canvas, &fonts);

                canvas.flush();
                #[cfg(not(target_arch = "wasm32"))]
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}
