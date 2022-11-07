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
    verts: [f32; 2],
}
glium::implement_vertex!(Vertex, verts);

pub const MESH: [Vertex; 4] = [
    Vertex {
        verts: [-0.5, -0.5],
    },
    Vertex {
        verts: [0.5, -0.5],
    },
    Vertex {
        verts: [0.5, 0.5],
    },
    Vertex {
        verts: [-0.5, 0.5],
    },
];
pub const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 verts;
    uniform vec2 pos;
    uniform mat4 view;
    void main() {
        gl_Position = vec4(verts + pos, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    out vec4 color;
    uniform vec3 candyColor;
    void main() {
        color = vec4(candyColor,1.0);
    }
"#;

pub const DEBUG_COLORS : [[f32;3]; 3] = [
    [1.0,0.0,0.0],
    [0.0,1.0,0.0],
    [0.0,0.0,1.0],
];