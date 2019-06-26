use glium::{glutin, implement_vertex, uniform, Surface};

fn main() {
    let mut event_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let shape = vec![
        Vertex {
            position: (-0.5, -0.5),
            color: (1.0, 0.0, 0.0),
            texture: (-0.5, -0.5),
        },
        Vertex {
            position: (0.0, 0.5),
            color: (0.0, 1.0, 0.0),
            texture: (0.0, 0.5),
        },
        Vertex {
            position: (0.5, -0.5),
            color: (0.0, 0.0, 1.0),
            texture: (0.5, -0.5),
        },
    ];

    let image = image::open("assets/wall.jpg").unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).expect("texture");

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader = include_str!("triangle.vert");
    let fragment_shader = include_str!("triangle.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();


    let mut exit = false;
    while !exit {
        event_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => exit = true,
                _ => (),
            },
            _ => (),
        });

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0_f32],
            ],
            tex: &texture
        };

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: (f32, f32),
    color: (f32, f32, f32),
    texture: (f32, f32),
}

implement_vertex!(Vertex, position, color, texture);
