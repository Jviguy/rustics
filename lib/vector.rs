use std::ops;

pub trait Vector {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mult(self, other: Self) -> Self;
    fn mult_scalar(self,scalar: f64) -> Self;
    fn div(self, other: Self) -> Self;
    fn div_scalar(self,scalar: f64) -> Self;
}


#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64, y: f64, z: f64
}

impl Vector for Vec3 {
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: other.x+self.x,
            y: other.y+self.y,
            z: other.z+self.z,
        }
    }
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: other.x-self.x,
            y: other.y-self.y,
            z: other.z-self.z,
        }
    }
    fn mult(self, other: Self) -> Self {
        Vec3 {
            x: other.x*self.x,
            y: other.y*self.y,
            z: other.z*self.z,
        }
    }
    fn mult_scalar(self, scalar: f64) -> Self {
        Vec3 {
            x: scalar*self.x,
            y: scalar*self.y,
            z: scalar*self.z,
        }
    }
    fn div(self, other: Self) -> Self {
        
    }
    fn div_scalar(self, scalar: f64) -> Self {

    }
}