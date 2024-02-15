use crate::math::{forces::Force, vector::Vector};

pub mod circle;

pub trait Object<T> {
    fn set_id(&mut self, id: i64);
    fn get_id(&self) -> i64;
    fn get_velocity(&self) -> Vector<T>;
    fn set_velocity(&mut self, velocity: Vector<T>);
    fn get_mass(&self) -> T;
    fn set_mass(&mut self, mass: T);
    fn get_acceleration(&self) -> Vector<T>;
    fn set_acceleration(&mut self, acceleration: Vector<T>);
    fn get_forces(&self) -> Vec<Force<T>>;
    fn remove_force(&mut self, force_idx: usize);
    fn add_force(&mut self, force: Force<T>);
}
