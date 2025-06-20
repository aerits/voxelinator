use crate::{obj::{Color, Mtl, MtlParams}, vec3::Vec3};

pub struct Cube {
    pub pos: Vec3,
    pub mat: Mtl,
    pub scale: Vec3,
    pub text_coord: Option<Vec<Vec3>>,
}
impl Cube {
    pub fn point_in_cube(&self, point: Vec3) -> bool {
        let (corner1, corner2) = {
            let corner1 = self.scale * 0.5;
            let corner2 = corner1 * -1.0;
            let corner1 = self.pos + corner1;
            let corner2 = self.pos + corner2;
            (corner1, corner2)
        };
        let small = |v1: f32, v2: f32| if v1 < v2 { v1 } else { v2 };
        let big = |v1: f32, v2: f32| if v1 < v2 { v2 } else { v1 };

        (small(corner1.x, corner2.x)..=big(corner1.x, corner2.x)).contains(&point.x)
            && (small(corner1.y, corner2.y)..=big(corner1.y, corner2.y)).contains(&point.y)
            && (small(corner1.z, corner2.z)..=big(corner1.z, corner2.z)).contains(&point.z)
    }
}
fn test_cube() {
    let c = Cube {
        pos: Vec3::from_float(0.0),
        mat: Mtl::builder(MtlParams {
            ambient_color: Color { r: 0.0, g: 0.0, b: 0.0 },
            diffuse_color: Color { r: 0.0, g: 0.0, b: 0.0 },
            specular_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        }).build(),
        scale: Vec3::from_float(1.0),
        text_coord: None,
    };
    let a = c.point_in_cube(Vec3::from_float(0.5));
    assert_eq!(true, a);
}