use core::f64;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use crate::math::vector::Vector;

impl Vector<i64> {
    pub fn magnitude(&self) -> i64 {
        (self.components.iter().map(|a| a.pow(2)).sum::<i64>() as f64).sqrt() as i64
    }

    pub fn normalized(self) -> Vector<i64> {
        let m = self.magnitude();
        self / m
    }

    pub fn distance(self, other: Vector<i64>) -> i64 {
        (self.distance_squared(other) as f64).sqrt() as i64
    }

    pub fn distance_squared(self, other: Vector<i64>) -> i64 {
        (self
            .components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| (a - b).pow(2))
            .sum::<i64>() as f64)
            .sqrt()
            .round() as i64
    }

    pub fn angle(self, other: Vector<i64>) -> i64 {
        let sm = self.magnitude();
        let om = other.magnitude();
        (((self * other) / (sm * om)) as f64).acos().round() as i64
    }

    pub fn max_components(vecs: Vec<Vector<i64>>) -> Vector<i64> {
        if vecs.is_empty() {
            return Vector::default();
        }

        Vector::new(vecs.iter().fold(
            vec![i64::MIN; vecs[0].components.len()],
            |mut max_values, vector| {
                vector
                    .components
                    .iter()
                    .enumerate()
                    .for_each(|(i, &value)| {
                        if value > max_values[i] {
                            max_values[i] = value;
                        }
                    });
                max_values
            },
        ))
    }

    pub fn min_components(vecs: Vec<Vector<i64>>) -> Vector<i64> {
        if vecs.is_empty() {
            return Vector::default();
        }

        Vector::new(vecs.iter().fold(
            vec![i64::MAX; vecs[0].components.len()],
            |mut min_values, vector| {
                vector
                    .components
                    .iter()
                    .enumerate()
                    .for_each(|(i, &value)| {
                        if value < min_values[i] {
                            min_values[i] = value;
                        }
                    });
                min_values
            },
        ))
    }

    pub fn add_component(&mut self, component: i64) {
        self.components.push(component)
    }

    pub fn remove_component(&mut self, dim: usize) {
        self.components.remove(dim);
    }
}
impl Add<Vector<i64>> for Vector<i64> {
    type Output = Vector<i64>;
    fn add(self, other: Self) -> Self {
        Vector::new(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl Sub<Vector<i64>> for Vector<i64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector::new(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl Mul<Vector<i64>> for Vector<i64> {
    type Output = i64;
    fn mul(self, other: Self) -> i64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl Mul<i64> for Vector<i64> {
    type Output = Vector<i64>;
    fn mul(self, scalar: i64) -> Self {
        Vector::new(self.components.iter().map(|a| a * scalar).collect())
    }
}
impl Div<Vector<i64>> for Vector<i64> {
    type Output = Vector<i64>;
    fn div(self, other: Self) -> Self {
        Vector::new(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a / b)
                .collect(),
        )
    }
}

impl Div<i64> for Vector<i64> {
    type Output = Vector<i64>;
    fn div(self, scalar: i64) -> Self {
        Vector::new(self.components.iter().map(|a| a * scalar).collect())
    }
}

impl Eq for Vector<i64> {}

impl PartialEq<Vector<i64>> for Vector<i64> {
    fn eq(&self, other: &Vector<i64>) -> bool {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a == b)
            .all(|x| x)
    }
}

impl Index<usize> for Vector<i64> {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vector<i64> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}
