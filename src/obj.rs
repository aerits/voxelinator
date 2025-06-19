use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use struct_builder::builder;

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Face {
    vertices: Vec<usize>,
    texture_coordinates: Option<Vec<usize>>,
    mtl: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[builder]
#[derive(Clone, Copy, PartialEq)]
pub struct Mtl {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub dissolve: Option<f32>, // transparency
    pub diffuse_texture_map: Option<usize>,
    pub illumination_model: Option<i32>,
}
impl Mtl {
    pub fn new_from_color(c: Color) -> Mtl {
        let params = MtlParams {
            ambient_color: c,
            diffuse_color: c,
            specular_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        Mtl::builder(params).build()
    }
}

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub materials: Vec<Mtl>,
    pub textures: Vec<String>,
}
impl Obj {
    pub fn new() -> Obj {
        Obj {
            vertices: Vec::new(),
            faces: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
            texture_coordinates: Vec::new(),
        }
    }
    pub fn new_cube(
        &mut self,
        position: Vec3,
        scale: f32,
        material: Mtl,
        texture_coordinates: Option<Vec3>,
    ) {
        let vertices = [
            (1.0, 1.0, 1.0),    // Vertex 1
            (1.0, 1.0, -1.0),   // Vertex 2
            (1.0, -1.0, 1.0),   // Vertex 3
            (1.0, -1.0, -1.0),  // Vertex 4
            (-1.0, 1.0, 1.0),   // Vertex 5
            (-1.0, 1.0, -1.0),  // Vertex 6
            (-1.0, -1.0, 1.0),  // Vertex 7
            (-1.0, -1.0, -1.0), // Vertex 8
        ];
        let first_vert = self.vertices.len();
        for vertex in vertices {
            let mut v = Vec3 {
                x: vertex.0,
                y: vertex.1,
                z: vertex.2,
            };
            v = v * 0.5;
            v = v * scale;
            v = v + position;
            self.vertices.push(v);
        }
        let tc = if let Some(x) = texture_coordinates {
            let first_text_coord = self.texture_coordinates.len();
            self.texture_coordinates.push(x);
            Some(vec![first_text_coord; 4])
        } else {
            None
        };
        let faces = [
            (1, 2, 4, 3), // Front face
            (5, 6, 8, 7), // Back face
            (1, 5, 7, 3), // Left face
            (2, 6, 8, 4), // Right face
            (1, 2, 6, 5), // Top face
            (3, 4, 8, 7), // Bottom face
        ];
        let new_mat = material;
        let mut mat = self.materials.len();
        let mut mat_found = false;
        for (i, m) in self.materials.iter().enumerate() {
            if m == &new_mat {
                mat = i;
                mat_found = true;
                break;
            }
        }
        if !mat_found {
            self.materials.push(new_mat);
        }

        for face in faces {
            let face = Face {
                vertices: vec![
                    face.0 + first_vert,
                    face.1 + first_vert,
                    face.2 + first_vert,
                    face.3 + first_vert,
                ],
                mtl: mat,
                texture_coordinates: tc.clone(),
            };
            self.faces.push(face);
        }
    }
    pub fn export_obj(&self, materials: PathBuf) -> String {
        let mut s = String::new();
        s += &format!("mtllib {:?}\n", materials);
        let pb = ProgressBar::new(self.vertices.len() as u64 + 1);
        for v in &self.vertices {
            s += &format!("v {} {} {}\n", v.x, v.y, v.z);
            pb.inc(1);
        }
        println!("finished vert");
        let pb3 = ProgressBar::new(self.vertices.len() as u64 + 1);
        for vt in &self.texture_coordinates {
            s += &format!("vt {} {}\n", vt.x, vt.y);
            pb3.inc(1);
        }
        println!("finished vt");
        let pb2 = ProgressBar::new(self.faces.len() as u64 + 1);
        {
            let mut last_face: Option<&Face> = None;
            for f in &self.faces {
                if last_face.is_none() || last_face.unwrap().mtl != f.mtl {
                    s += &format!("usemtl m{}\n", f.mtl);
                }
                s += "f ";
                if let Some(x) = &f.texture_coordinates {
                    for (v, t) in x.iter().zip(&f.vertices) {
                        s += &format!(" {}/{} ", v, t);
                    }
                } else {
                    for i in &f.vertices {
                        s += &format!(" {} ", i);
                    }
                }
                s += "\n";
                last_face = Some(f);
                pb2.inc(1);
            }
            println!("finished face");
        }

        return s;
    }
    pub fn export_mtl(&self) -> String {
        let mut s = String::new();
        let write_color = |n: &str, c: Color| {
            return format!(
                "{} {} {} {}\n",
                n,
                format_float(c.r),
                format_float(c.g),
                format_float(c.b)
            );
        };
        for (i, m) in self.materials.iter().enumerate() {
            s += format!("newmtl m{}\n", i).as_str();
            s += &write_color("Ka", m.ambient_color);
            s += &write_color("Kd", m.diffuse_color);
            s += &write_color("Ks", m.specular_color);
            if let Some(x) = m.dissolve {
                s += format!("d {}\n", format_float(x)).as_str();
            }
            if let Some(x) = m.illumination_model {
                s += format!("illum {}\n", x).as_str();
            }
            if let Some(x) = m.diffuse_texture_map {
                s += format!("map_Kd {}\n", self.textures[x]).as_str();
            }
        }
        return s;
    }
}

fn format_float(f: f32) -> String {
    if f.fract() == 0.0 {
        format!("{:.1}", f) // Show one decimal place for whole numbers
    } else {
        format!("{}", f) // Show full precision for non-whole numbers
    }
}
