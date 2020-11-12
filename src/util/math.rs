use std::ops::Sub;


/// represents a point a 3D environment
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}


impl Vec3 {

    pub fn from(pos: [f32; 3]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2]
        }
    }

    pub fn distance(a: Vec3, b: Vec3) -> f32 {
        let vector = a - b;

        // return the eclidian distance of the vector
        f32::sqrt(
            vector.x.powi(2) +
                vector.y.powi(2) +
                vector.z.powi(2)
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
