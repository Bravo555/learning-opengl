pub const SHAPE: [Vertex; 36] = [
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (0.0, 0.0, -1.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        normal: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        normal: (-1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        normal: (0.0, -1.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        normal: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        normal: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        normal: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        normal: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        normal: (0.0, 1.0, 0.0),
    },
];

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
}
