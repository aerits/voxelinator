use std::error::Error;
use std::io::{self, Write};
use std::{fs::File, path::PathBuf};

use voxelinator::{
    obj::{Color, Mtl, MtlParams, Obj},
    vec3::Vec3,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut model = Obj::new();
    model.textures.push("bruh.png".to_string());
    let white = Color {
        r: 1.0,
        b: 1.0,
        g: 1.0,
    };
    let mat = Mtl::builder(MtlParams {
        ambient_color: white,
        diffuse_color: white,
        specular_color: white,
    })
    .with_dissolve(Some(1.0))
    .with_diffuse_texture_map(Some(0))
    .build();
    model.new_cube(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        1.0,
        mat,
        Some(vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
        ]),
    );
    println!(
        "vertices: {}\nfaces {}\n materials {}",
        model.vertices.len(),
        model.faces.len(),
        model.materials.len()
    );

    let mut file = File::create("bruh".to_owned() + ".mtl")?;
    let s = model.export_mtl();
    writeln!(file, "{}", s)?;

    let s = model.export_obj(PathBuf::from("bruh".to_owned() + ".mtl"));
    let mut file = File::create("bruh".to_owned() + ".obj")?;
    writeln!(file, "{}", s)?;
    println!("wrote to file");
    Ok(())
}
