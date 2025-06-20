use image::{DynamicImage, GenericImageView};
use indicatif::{ProgressBar, ProgressStyle};
use obj::{Mtl, MtlParams};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::path::PathBuf;

use structopt::StructOpt;

use crate::cube::Cube;
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

mod cube;
mod obj;
mod vec3;

fn u8_to_float(color: u8) -> f32 {
    color as f32 / 255.0
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("{:?}, {:?}", opt.input, opt.output);
    let mut model = Obj::new();
    model.textures.push(
        opt.input
            .clone()
            .into_os_string()
            .to_str()
            .unwrap()
            .to_string(),
    );

    let img = image::open(opt.input.clone())?;
    let size = img.pixels().size_hint();
    let pb = ProgressBar::new(size.0 as u64);

    let mut cubes = get_cubes(pb, img);

    println!("processing pixels into cubes");
    // Iterate through each pixel

    let pb = ProgressBar::new(cubes.len() as u64);
    println!("optimizing cubes");
    optimize_cubes(pb, &mut cubes);

    println!("creating cubes");
    let pb = ProgressBar::new(cubes.len() as u64);
    for cube in cubes {
        model.new_rect(
            cube.pos,
            cube.scale.x,
            cube.scale.y,
            cube.scale.z,
            cube.mat,
            cube.text_coord,
        );
        pb.inc(1);
    }
    pb.finish_and_clear();

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

fn optimize_cubes(pb: ProgressBar, cubes: &mut Vec<Cube>) {
    let up = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let down = up * -1.0;
    let right = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let left = right * -1.0;
    let forward = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let backward = forward * -1.0;
    let dirs = vec![up, down, right, left, forward, backward];
    for cube in cubes {
        // check if each direction if you can expand the current cube to there
        // being expandable to there means that if you extend the cube in that direction, it doesn't fill in any holes
        pb.inc(1);
    }
    pb.finish_and_clear();
}

fn get_cubes(pb: ProgressBar, img: DynamicImage) -> Vec<Cube> {
    let mut cubes = Vec::new();

    let img_height = img.height();
    let img_width = img.width();
    let img = img.to_rgba8(); // Convert to RGBA format
    for (x, y, pixel) in img.enumerate_pixels() {
        let rgba = pixel.0; // Get the RGBA values
        if rgba[3] == 0 {
            continue;
        }
        let white = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        let params = MtlParams {
            ambient_color: white,
            diffuse_color: white,
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
        let c = Cube {
            pos: Vec3 {
                x: x as f32,
                y: y as f32 * -1.0,
                z: 0.0,
            },
            scale: Vec3::from_float(1.0),
            mat,
            text_coord: Some(vec![icord; 4]),
        };
        cubes.push(c);
        pb.inc(1);
    }
    pb.finish_and_clear();
    return cubes;
}
