mod points;

use glium::{glutin, implement_vertex, uniform, Surface};
use nalgebra as na;
use points::{Vertex, SHAPE};
use std::collections::HashMap;
use std::time::Instant;
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

    let projection: [[f32; 4]; 4] =
        na::Perspective3::new(width as f32 / height as f32, 45.0, 0.1, 100.0)
            .into_inner()
            .into();

    let last_time = Instant::now();

    let mix_level: f32 = 0.5;
    let mut exit = false;
    let camera_pos = na::Vector3::new(0.0, 0.0, 3.0);
    let camera_front = na::Vector3::new(0.0, 0.0, -1.0);
    let up = na::Vector3::y();
    let mut camera = Camera::new(camera_pos, camera_front, up);
    let mut input_buffer: HashMap<glutin::VirtualKeyCode, glutin::ElementState> = HashMap::new();

    while !exit {
        let _time_since: f32 = last_time.elapsed().as_millis() as f32 / 1000.0;

        event_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => exit = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    process_input(input, &mut input_buffer, &mut camera)
                }
                _ => (),
            },
            _ => (),
        });

        camera.update();

        let view = camera.view();
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

fn process_input(
    input: glutin::KeyboardInput,
    input_buffer: &mut HashMap<glutin::VirtualKeyCode, glutin::ElementState>,
    camera: &mut Camera,
) {
    println!("{:#?}", input);
    let camera_speed = 0.005;
    let mut new_camera_velocity = na::Vector3::<f32>::identity();

    if let Some(key) = input.virtual_keycode {
        use glutin::VirtualKeyCode;

        if input.state
            == *input_buffer
                .entry(key)
                .or_insert(glutin::ElementState::Released)
        {
            return;
        }
        input_buffer.insert(key, input.state);

        match key {
            VirtualKeyCode::W => new_camera_velocity = camera.front() * camera_speed,
            VirtualKeyCode::S => new_camera_velocity = -camera.front() * camera_speed,
            VirtualKeyCode::A => {
                new_camera_velocity = -camera.front().cross(&camera.up()).normalize() * camera_speed
            }
            VirtualKeyCode::D => {
                new_camera_velocity = camera.front().cross(&camera.up()).normalize() * camera_speed
            }
            _ => (),
        }
    }
    camera.velocity += if let glutin::ElementState::Pressed = input.state {
        new_camera_velocity
    } else {
        -new_camera_velocity
    };
}

struct Camera {
    position: na::Vector3<f32>,
    front: na::Vector3<f32>,
    up: na::Vector3<f32>,
    velocity: na::Vector3<f32>,
}

impl Camera {
    fn new(position: na::Vector3<f32>, front: na::Vector3<f32>, up: na::Vector3<f32>) -> Camera {
        Camera {
            position,
            front,
            up,
            velocity: na::Vector3::<f32>::zeros(),
        }
    }

    fn target(&self) -> na::Point3<f32> {
        na::Point3::from(self.position + self.front)
    }

    fn view(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&na::Point3::from(self.position), &self.target(), &self.up)
    }

    fn front(&self) -> na::Vector3<f32> {
        self.front
    }

    fn up(&self) -> na::Vector3<f32> {
        self.up
    }

    fn translate(&mut self, offset: na::Vector3<f32>) {
        self.position += offset
    }

    fn update(&mut self) {
        self.translate(self.velocity)
    }
}