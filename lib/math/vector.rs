use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone)]
pub struct Vector<T> {
    pub components: Vec<T>,
}

impl Vector<f64> {
    pub fn new<T>(components: Vec<T>) -> Vector<T> {
        Vector { components }
    }

    pub fn magnitude(&self) -> f64 {
        self.components
            .iter()
            .map(|a| a.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn normalized(self) -> Vector<f64> {
        let m = self.magnitude();
        self / m
    }

    pub fn distance(self, other: Vector<f64>) -> f64 {
        self.distance_squared(other).sqrt()
    }

    pub fn distance_squared(self, other: Vector<f64>) -> f64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn angle(self, other: Vector<f64>) -> f64 {
        let sm = self.magnitude();
        let om = other.magnitude();
        ((self * other) / (sm * om)).acos()
    }

    pub fn max_components(vecs: Vec<Vector<f64>>) -> Vector<f64> {
        if vecs.is_empty() {
            return Vector::default();
        }

        Vector::new(vecs.iter().fold(
            vec![f64::MIN; vecs[0].components.len()],
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

    pub fn min_components(vecs: Vec<Vector<f64>>) -> Vector<f64> {
        if vecs.is_empty() {
            return Vector::default();
        }

        Vector::new(vecs.iter().fold(
            vec![f64::MAX; vecs[0].components.len()],
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

    pub fn add_component(&mut self, component: f64) {
        self.components.push(component)
    }

    pub fn remove_component(&mut self, dim: usize) {
        self.components.remove(dim);
    }

    pub fn div_scalar(&self, scalar: f64) -> Vector<f64> {
        Vector::new(self.components.iter().map(|a| a * scalar).collect())
    }
}
impl Add<Vector<f64>> for Vector<f64> {
    type Output = Vector<f64>;
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

impl Sub<Vector<f64>> for Vector<f64> {
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

impl Mul<Vector<f64>> for Vector<f64> {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl Mul<f64> for Vector<f64> {
    type Output = Vector<f64>;
    fn mul(self, scalar: f64) -> Self {
        Vector::new(self.components.iter().map(|a| a * scalar).collect())
    }
}
impl Div<Vector<f64>> for Vector<f64> {
    type Output = Vector<f64>;
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

impl Div<f64> for Vector<f64> {
    type Output = Vector<f64>;
    fn div(self, scalar: f64) -> Self {
        Vector::new(self.components.iter().map(|a| a * scalar).collect())
    }
}

impl Eq for Vector<f64> {}

impl PartialEq<Vector<f64>> for Vector<f64> {
    fn eq(&self, other: &Vector<f64>) -> bool {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a == b)
            .all(|x| x)
    }
}

impl Index<usize> for Vector<f64> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vector<f64> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}
