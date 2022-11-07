pub struct Candy {
    pub pos: [f32; 2],
    pub t: Type,
}
pub enum Type {
    Normal(Color),
    Striped(Color, Direction),
    Wrapped(Color),
    ColourBomb,
    ColouringCandy(Color),
    JellyFish(Color),
    BlackCandy,
}
pub enum Direction {
    Horizontal,
    Vertical,
}

pub type Color = usize;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

pub const MESH: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5],
    },
    Vertex {
        position: [0.0, 0.5],
    },
];
pub const INDICES: [u16; 3] = [0u16, 1, 2];

pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 pos;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    out vec4 color;
    void main() {
        color = vec4(1.0,1.0,0.0,1.0);
    }
"#;

pub const DEBUG_COLORS : [[f32;3]; 3] = [
    [1.0,1.0,1.0],
    [1.0,1.0,1.0],
    [1.0,1.0,1.0],
];