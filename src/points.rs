pub const SHAPE: [Vertex; 36] = [
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, -0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5, -0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        texture: (0.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, -0.5),
        texture: (1.0, 1.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (0.5, 0.5, 0.5),
        texture: (1.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, 0.5),
        texture: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5, 0.5, -0.5),
        texture: (0.0, 1.0),
    },
];

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub texture: (f32, f32),
}
