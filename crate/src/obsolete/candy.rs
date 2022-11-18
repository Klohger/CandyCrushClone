use cgmath::Matrix4;

#[derive(Clone, Copy)]
pub struct Candy {
    pub matrix: Matrix4<f32>,
    pub t: Type,
}
#[derive(Clone, Copy)]
pub enum Type {
    Normal(Color),
    Striped(Color, Direction),
    Wrapped(Color),
    ColourBomb,
    ColouringCandy(Color),
    JellyFish(Color),
    BlackCandy,
}
#[derive(Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub type Color = usize;

#[derive(Clone, Copy)]
pub struct Vertex {
    verts: [f32; 2],
    uv_coords: [f32; 2],
}
glium::implement_vertex!(Vertex, verts, uv_coords);

pub const MESH: [Vertex; 4] = [
    Vertex {
        verts: [-0.5, -0.5],
        uv_coords: [0.0, 0.0],
    },
    Vertex {
        verts: [0.5, -0.5],
        uv_coords: [1.0, 0.0],
    },
    Vertex {
        verts: [0.5, 0.5],
        uv_coords: [1.0, 1.0],
    },
    Vertex {
        verts: [-0.5, 0.5],
        uv_coords: [0.0, 1.0],
    },
];
pub const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 verts;
    in vec2 uv_coords;
    out vec2 texCoords;

    uniform mat4 mat;
    uniform mat4 projection;
    uniform mat4 camera;
    void main() {
        texCoords = uv_coords;
        gl_Position = projection * camera * mat * vec4(verts, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    
    in vec2 texCoords;
    out vec4 color;

    uniform sampler2DArray tex;
    uniform float colorId;

    void main() {
        vec4 texColor = texture(tex, vec3(texCoords, colorId));


        if(texColor.a < 0.5) {
            discard;
        }
        color = texColor;
    }
"#;