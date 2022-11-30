use cgmath::num_traits::ToPrimitive;
use glium::{VertexBuffer, IndexBuffer, implement_vertex};
use obj::FromRawVertex;

pub struct Mesh<V : FromRawVertex<I> + glium::vertex::Vertex = Vertex, I : glium::index::Index = u16> {
    pub vertex_buffer : VertexBuffer<V>,
    pub index_buffer : IndexBuffer<I>,
}
#[derive(Clone, Copy)]
    pub struct Vertex {
        verts: [f32; 3],
        uv_coords: [f32; 2],
        normals: [f32; 3],
    }
    implement_vertex!(Vertex, verts, uv_coords, normals);
    impl FromRawVertex<u16> for Vertex {
        fn process(
            positions: Vec<(f32, f32, f32, f32)>,
            normals: Vec<(f32, f32, f32)>,
            tex_coords: Vec<(f32, f32, f32)>,
            polygons: Vec<obj::raw::object::Polygon>,
        ) -> obj::ObjResult<(Vec<Self>, Vec<u16>)> {
            let mut vb = Vec::with_capacity(polygons.len() * 3);
            let mut ib = Vec::with_capacity(polygons.len() * 3);
            {
                let mut cache = std::collections::HashMap::new();
                let mut map = |pi: usize, ti: usize, ni: usize| {
                    // Look up cache
                    let index = match cache.entry((pi, ti, ni)) {
                        // Cache miss -> make new, store it on cache
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            let p = positions[pi];
                            let t = tex_coords[ti];
                            let n = normals[ni];
                            let vertex = Vertex {
                                verts: [p.0, p.1, p.2],
                                uv_coords: [t.0, t.1],
                                normals: [n.0, n.1, n.2],
                            };
                            let index = vb
                                .len()
                                .to_u16()
                                .expect("Unable to convert the index from usize");
                            vb.push(vertex);
                            entry.insert(index);
                            index
                        }
                        // Cache hit -> use it
                        std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
                    };
                    ib.push(index)
                };

                for polygon in polygons {
                    match polygon {
                        obj::raw::object::Polygon::P(_) 
                        | obj::raw::object::Polygon::PT(_) => panic!("Tried to extract normal data which are not contained in the model"),
                        obj::raw::object::Polygon::PN(_) => panic!("Tried to extract texture coordinate data which are not contained in the model"),
                        obj::raw::object::Polygon::PTN(ref vec) if vec.len() == 3 => {
                            for &(pi, ti, ni) in vec {
                                map(pi, ti, ni)
                            }
                        }
                        _ => panic!("Model should be triangulated first to be loaded properly"),
                    }
                }


            }
            vb.shrink_to_fit();
            Ok((vb, ib))
        }
    }