use indicatif::{ProgressBar, ProgressStyle};
use obj::{Mtl, MtlParams};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::path::PathBuf;

use structopt::StructOpt;

use crate::obj::{Color, Obj};
use crate::vec3::Vec3;

/// converts a folder of pixel art into a voxel model
#[derive(StructOpt)]
struct Opt {
    /// input folder
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    /// output file
    #[structopt(short, long)]
    output: String,
}

mod obj;
mod vec3;

fn u8_to_float(color: u8) -> f32 {
    color as f32 / 255.0
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("{:?}, {:?}", opt.input, opt.output);
    let mut model = Obj::new();
    model
        .textures
        .push(opt.input.clone().into_os_string().to_str().unwrap().to_string());

    let img = image::open(opt.input.clone())?;
    let img_height = img.height();
    let img_width = img.width();
    let img = img.to_rgba8(); // Convert to RGBA format
    let pb = ProgressBar::new(img.enumerate_pixels().len() as u64);

    // Iterate through each pixel
    for (x, y, pixel) in img.enumerate_pixels() {
        let rgba = pixel.0; // Get the RGBA values
        let c = Color {
            r: u8_to_float(rgba[0]),
            g: u8_to_float(rgba[1]),
            b: u8_to_float(rgba[2]),
        };
        let white = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        let params = MtlParams {
            ambient_color: c,
            diffuse_color: c,
            specular_color: white,
        };
        let mat = Mtl::builder(params)
            .with_diffuse_texture_map(Some(0))
            .build();
        let icord = Vec3 {
            x: x as f32 / img_width as f32,
            y: y as f32 * -1.0 / img_height as f32,
            z: 0.0,
        };
        model.new_cube(
            Vec3 {
                x: x as f32,
                y: y as f32 * -1.0,
                z: 0.0,
            } * 0.1,
            0.1,
            mat,
            // None,
            None,
        );
        pb.inc(1);
    }
    println!("finished processing image");
    println!(
        "vertices: {}\nfaces {}\n materials {}",
        model.vertices.len(),
        model.faces.len(),
        model.materials.len()
    );

    let mut file = File::create(opt.output.clone() + ".mtl")?;
    let s = model.export_mtl();
    writeln!(file, "{}", s)?;

    let s = model.export_obj(PathBuf::from(opt.output.clone() + ".mtl"));
    let mut file = File::create(opt.output.clone() + ".obj")?;
    writeln!(file, "{}", s)?;
    println!("wrote to file");
    Ok(())
}
