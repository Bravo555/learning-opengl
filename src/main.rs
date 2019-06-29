mod points;

use glium::{glutin, implement_vertex, uniform, Surface};
use nalgebra as na;
use points::{Vertex, SHAPE};
use std::time::{Duration, Instant};
implement_vertex!(Vertex, position, texture);

fn main() {
    let vertex_shader = include_str!("triangle.vert");
    let fragment_shader = include_str!("triangle.frag");

    let mut event_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let texture = load_texture("assets/wall.jpg", &display);
    let texture2 = load_texture("assets/face.png", &display);

    let models: Vec<na::Matrix4<f32>> = vec![
        (0.0, 0.0, 0.0),
        (2.0, 5.0, -15.0),
        (-1.5, -2.2, -2.5),
        (-3.8, -2.0, -12.3),
        (2.4, -0.4, -3.5),
        (-1.7, 3.0, -7.5),
        (1.3, -2.0, -2.5),
        (1.5, 2.0, -2.5),
        (1.5, 0.2, -1.5),
        (-1.3, 1.0, -1.5),
    ]
    .into_iter()
    .map(|m| na::Matrix4::new_translation(&na::Vector3::new(m.0, m.1, m.2)))
    .collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, &SHAPE).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    let (width, height) = display.get_framebuffer_dimensions();

    let radius = 10.0;

    let view: [[f32; 4]; 4] =
        na::Matrix4::new_translation(&na::Vector3::new(0.0, 0.0, -3.0)).into();
    let projection: [[f32; 4]; 4] =
        na::Perspective3::new(width as f32 / height as f32, 45.0, 0.1, 100.0)
            .into_inner()
            .into();

    let last_time = Instant::now();

    let mut mix_level: f32 = 0.5;
    let mut exit = false;
    while !exit {
        event_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => exit = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let glutin::ElementState::Pressed = input.state {
                        let mix_add = if let Some(key) = input.virtual_keycode {
                            match key {
                                glutin::VirtualKeyCode::Up => 0.1,
                                glutin::VirtualKeyCode::Down => -0.1,
                                _ => 0.0,
                            }
                        } else {
                            0.0
                        };

                        mix_level = (mix_level + mix_add).min(1.0).max(0.0);
                    }
                }
                _ => (),
            },
            _ => (),
        });

        let time_since: f32 = last_time.elapsed().as_millis() as f32 / 1000.0;

        let cam_x = time_since.sin() * radius;
        let cam_z = time_since.cos() * radius;
        let camera_pos = na::Point3::new(cam_x, 0.0, cam_z);
        let target = na::Point3::origin();
        let up = na::Vector3::y();

        let view = na::Matrix4::look_at_rh(&camera_pos, &target, &up);
        let view: [[f32; 4]; 4] = view.into();

        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        models.iter().enumerate().for_each(|(i, val)| {
            let angle = 20.0 * i as f32;
            let rotation = na::Rotation3::new(na::Vector3::new(1.0, 0.3, 0.5) * angle.to_radians())
                .to_homogeneous();

            let model: [[f32; 4]; 4] = (val * rotation).into();

            let uniforms = uniform! {
                model: model,
                view: view,
                projection: projection,
                tex: &texture,
                tex2: &texture2,
                mix_level: mix_level
            };
            target
                .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
                .unwrap();
        });

        target.finish().unwrap();
    }
}

fn load_texture<D>(path: &str, display: &D) -> glium::texture::Texture2d
where
    D: glium::backend::Facade,
{
    let image = image::open(path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).expect("texture")
}
