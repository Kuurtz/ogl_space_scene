mod shapes;

#[macro_use]
extern crate glium;

extern crate image;

use std::any::Any;
use std::f32::consts::PI;
use glium::{Display, glutin, Surface};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::event::Event;
use glium::glutin::event_loop::EventLoop;
use glium::texture::*;
use shapes::matrices;
use crate::glutin::event_loop::ControlFlow;
use crate::shapes::{DynDrawble, StaticDrawble, Transform};

type Light = [f32; 3];

fn start_opengl(
    title: &str,
    mut size: Option<(u32, u32)>,
) -> (EventLoop<()>, Result<Display, DisplayCreationError>) {
    let size = size.get_or_insert((400, 400));

    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(glutin::dpi::LogicalSize::new(size.0, size.1));

    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &event_loop);
    (event_loop, display)
}

fn main() {
    let (event_loop, display) = match start_opengl("First", None) {
        (event_loop, Ok(display)) => (event_loop, display),
        (_, Err(e)) => panic!("Could not create window: {e}"),
    };

    let moon_texture = load_tex!(display, "imgs/2k_venus_surface.jpg", jpeg);
    let earth_texture = load_tex!(display, "imgs/2k_earth_daymap.jpg", jpeg);
    // let image = image::load(std::io::Cursor::new(&include_bytes!("imgs/2k_earth_daymap.jpg")),
    //                         image::ImageFormat::Jpeg).unwrap().to_rgba8();

    // let image_dimensions = image.dimensions();
    // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    // let earth_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let earth = shapes::sphere::SphereBuilder::new()
        .radius(1.0)
        .texture(earth_texture)
        .build(&display);

    // let image = image::load(std::io::Cursor::new(&include_bytes!("imgs/2k_venus_surface.jpg")),
    //                         image::ImageFormat::Jpeg).unwrap().to_rgba8();
    //
    // let image_dimensions = image.dimensions();
    // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    //
    // let moon_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    let moon = shapes::sphere::SphereBuilder::new()
        .radius(0.1)
        // .texture(moon_texture)
        .color([0.5; 3])
        .build(&display);

    let asteroid = shapes::cube::CubeBuilder::new()
        .size(0.5)
        .color([0.2; 3])
        .build(&display);

    let mut sky = shapes::sky::Sky::new(&display);

    let draw_params = glium::draw_parameters::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            range: (0.0, 0.9),
            ..Default::default()
        },
        // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    // Render runtime
    let mut angle = (0..360)
        .map(|i| i as f32 * PI / 180.0) // transform to radians
        .cycle();

    let mut size = (0..240) // frames
        .map(|i| (i - 120) as f32 * 0.3 / 240.0 + 0.4)
        .cycle();

    event_loop.run(move |ev, _, cf| {
        let a = angle.next().unwrap();
        let s = size.next().unwrap();
        let mut target = display.draw();

        target.clear_color_and_depth((0., 0., 0., 1.), 1.);

        set_wait(cf, 16_666_667);

        event_handle(ev, cf);

        let perspective = matrices::perspective_matrix(&mut target);

        earth.draw(&mut target, &draw_params, Transform {
            rotate_self: [0.0, a, 0.0],
            scale: 0.3,
            ..Default::default()
        });

        moon.draw(&mut target, &draw_params, Transform {
            translation: [-0.8, 0.0, 0.0],
            rotate_self: [0.0, a, 0.0],
            rotation: [0.0, a, a.cos() * 0.4],
            ..Default::default()
        });

        asteroid.draw(&mut target, &draw_params, Transform {
            translation: [0.5, 0.5, 0.5],
            rotate_self: [0.0, a, 0.2],
            scale: 0.25,
            ..Default::default()
        });

        sky.draw(&mut target, &draw_params);

        target.finish().unwrap();
    })
}

fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}

fn event_handle(ev: Event<()>, cf: &mut ControlFlow) {
    match ev {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *cf = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }
}
