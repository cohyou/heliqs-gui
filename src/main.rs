mod event_loop;

// --- std ---
use std::{
    path::Path,
    // sync::{Arc, Mutex},
};
// --- external ---
use conrod::backend::glium::glium::{self, Surface, glutin};

pub fn display() {
    use self::event_loop::EventLoop;

    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 800;

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("heliqs-gui")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    {
        if cfg!(target_os = "windows") {
            ui.fonts.insert_from_file(Path::new("C:/Windows/Fonts/SIMFANG.ttf")).unwrap();
        } else {
            ui.fonts.insert_from_file(Path::new("/Library/Fonts/Arial Unicode.ttf")).unwrap();
        };
    }

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    #[allow(unused_mut)] let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested | glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => ()
                }
                _ => ()
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0., 0., 0., 1.);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn main() {
    display();
}
