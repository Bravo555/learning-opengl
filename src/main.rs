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

    let mut last_frame = 0.0;

    display.gl_window().window().grab_cursor(true).unwrap();
    display.gl_window().window().hide_cursor(true);

    let mut keyboard_state = KeyboardState::new();

    while !exit {
        let current_frame: f32 = last_time.elapsed().as_millis() as f32 / 1000.0;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        event_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => exit = true,
                _ => (),
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                glutin::DeviceEvent::MouseMotion { delta, .. } => {
                    handle_mouse_move(delta, &mut camera, delta_time);
                }
                glutin::DeviceEvent::Key(event) => keyboard_state.process_event(event),
                glutin::DeviceEvent::MouseWheel { delta } => {}
                _ => (),
            },
            _ => (),
        });
        process_keyboard(&keyboard_state, &mut camera, delta_time);

        camera.update(delta_time);

        let view: [[f32; 4]; 4] = camera.view().into();

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

fn process_keyboard(keyboard: &KeyboardState, camera: &mut Camera, delta_time: f32) {
    let camera_speed = 2.5 * delta_time;
    use glutin::VirtualKeyCode;

    if keyboard.is_pressed(&VirtualKeyCode::W) {
        camera.translate(camera.front() * camera_speed);
    }
    if keyboard.is_pressed(&VirtualKeyCode::S) {
        camera.translate(-camera.front() * camera_speed);
    }
    if keyboard.is_pressed(&VirtualKeyCode::A) {
        camera.translate(-camera.front().cross(&camera.up()).normalize() * camera_speed);
    }
    if keyboard.is_pressed(&VirtualKeyCode::D) {
        camera.translate(camera.front().cross(&camera.up()).normalize() * camera_speed);
    }
}

fn handle_mouse_move(position: (f64, f64), camera: &mut Camera, delta_time: f32) {
    let sensitivity = 25.0 * delta_time as f64;
    let offset_x = sensitivity * position.0;
    let offset_y = sensitivity * position.1 * -1.0; // y increases as mouse is moving down so mouse down = pitch up. -1 inverts that
    camera.rotate(offset_y as f32, offset_x as f32);
}

#[derive(Debug)]
struct Camera {
    position: na::Vector3<f32>,
    front: na::Vector3<f32>,
    up: na::Vector3<f32>,

    yaw: f32,
    pitch: f32,

    fov: f32,
}

impl Camera {
    fn new(position: na::Vector3<f32>, front: na::Vector3<f32>, up: na::Vector3<f32>) -> Camera {
        Camera {
            position,
            front,
            up,

            yaw: 270.0,
            pitch: 0.0,

            fov: 45.0,
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

    fn rotate(&mut self, pitch: f32, yaw: f32) {
        self.pitch = (self.pitch + pitch).min(89.0).max(-89.0);
        self.yaw += yaw;
    }

    fn update(&mut self, delta_time: f32) {
        // recalculate rotation
        let pitch = self.pitch.to_radians();
        let yaw = self.yaw.to_radians();
        let dir_x = pitch.cos() * yaw.cos();
        let dir_y = pitch.sin();
        let dir_z = pitch.cos() * yaw.sin();

        let front = na::Vector3::new(dir_x, dir_y, dir_z);
        self.front = front.normalize();
    }
}

struct KeyboardState {
    state: HashMap<glutin::VirtualKeyCode, glutin::ElementState>,
}

impl KeyboardState {
    fn new() -> KeyboardState {
        KeyboardState {
            state: HashMap::new(),
        }
    }

    fn is_pressed(&self, key: &glutin::VirtualKeyCode) -> bool {
        self.state
            .get(key)
            .map(|&s| s == glutin::ElementState::Pressed)
            .unwrap_or(false)
    }

    fn is_released(&self, key: &glutin::VirtualKeyCode) -> bool {
        !self.is_pressed(key)
    }

    fn process_event(&mut self, event: glutin::KeyboardInput) {
        if let Some(key) = event.virtual_keycode {
            match event.state {
                glutin::ElementState::Pressed => self.state.insert(key, event.state),
                glutin::ElementState::Released => self.state.remove(&key),
            };
        }
    }
}
