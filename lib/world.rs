use crate::{
    math::{forces::Force, vector::Vector},
    object::Object,
};

// Represents a physics space.
pub struct World<T: Object<f64> + PartialEq> {
    pub objects: Vec<T>,
    id: i64,
    gravity: Vector<f64>,
}

impl<T: Object<f64> + PartialEq> World<T> {
    pub fn new(gravity: Vector<f64>) -> World<T> {
        World {
            id: 0,
            objects: vec![],
            gravity,
        }
    }

    pub fn add_object(&mut self, mut object: T) {
        object.set_id(self.id);
        self.id += 1;
        self.objects.push(object)
    }

    pub fn remove_object(&mut self, object: T) {
        self.objects
            .remove(self.objects.iter().position(|f| *f == object).unwrap());
    }

    pub fn tick(&mut self) {
        for obj in &mut self.objects {
            let mut sum: Vector<f64> = Vector { components: vec![] };
            for force in obj.get_forces() {
                sum = sum + force.force;
            }
            sum = sum + self.gravity.clone();
            obj.set_acceleration(sum.div_scalar(obj.get_mass()));
        }
    }
}
