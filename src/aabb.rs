use crate::Orientation;
use std::fmt::Debug;

pub enum PlaneAxis {
    X,
    Y,
    Z,
}

pub struct Plane(f64, PlaneAxis);

#[derive(Debug, Clone)]
pub struct AABB {
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    pub orientation: Orientation,
}

#[inline]
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        b
    } else {
        a
    }
}

#[inline]
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

impl AABB {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        Self {
            x1: min(x1, x2),
            y1: min(y1, y2),
            z1: min(z1, z2),
            x2: max(x2, x1),
            y2: max(y1, y2),
            z2: max(z1, z2),
            orientation: Orientation::N,
        }
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn slice(self, plane: Plane) -> Vec<Self> {
        let orientation = self.orientation;
        match plane.1 {
            PlaneAxis::X => {
                if self.x1 < plane.0 && self.x2 > plane.0 {
                    vec![
                        Self::new(self.x1, self.y1, self.z1, plane.0, self.y2, self.z2)
                            .with_orientation(orientation | Orientation::L),
                        Self::new(plane.0, self.y1, self.z1, self.x2, self.y2, self.z2)
                            .with_orientation(orientation | Orientation::R),
                    ]
                } else if self.x2 <= plane.0 {
                    vec![self.with_orientation(orientation | Orientation::L)]
                } else {
                    vec![self.with_orientation(orientation | Orientation::R)]
                }
            }
            PlaneAxis::Y => {
                if self.y1 < plane.0 && self.y2 > plane.0 {
                    vec![
                        Self::new(self.x1, self.y1, self.z1, self.x2, plane.0, self.z2)
                            .with_orientation(orientation | Orientation::D),
                        Self::new(self.x1, plane.0, self.z1, self.x2, self.y2, self.z2)
                            .with_orientation(orientation | Orientation::U),
                    ]
                } else if self.y2 <= plane.0 {
                    vec![self.with_orientation(orientation | Orientation::D)]
                } else {
                    vec![self.with_orientation(orientation | Orientation::U)]
                }
            }
            PlaneAxis::Z => {
                if self.z1 < plane.0 && self.z2 > plane.0 {
                    vec![
                        Self::new(self.x1, self.y1, self.z1, self.x2, self.y2, plane.0)
                            .with_orientation(orientation | Orientation::B),
                        Self::new(self.x1, self.y1, plane.0, self.x2, self.y2, self.z2)
                            .with_orientation(orientation | Orientation::F),
                    ]
                } else if self.z2 <= plane.0 {
                    vec![self.with_orientation(orientation | Orientation::B)]
                } else {
                    vec![self.with_orientation(orientation | Orientation::F)]
                }
            }
        }
    }

    pub fn explode(self, center: (f64, f64, f64)) -> Vec<Self> {
        self.slice(Plane(center.0, PlaneAxis::X))
            .into_iter()
            .map(|aabb| aabb.slice(Plane(center.1, PlaneAxis::Y)))
            .flatten()
            .map(|aabb| aabb.slice(Plane(center.2, PlaneAxis::Z)))
            .flatten()
            .collect()
    }

    pub fn fit_in(&self, depth: u32, max_depth: u32) -> bool {
        let edge_size = 1_f64 / (2_usize.pow(depth) as f64);
        (((self.x1 - self.x2).abs() - edge_size).abs() < std::f64::EPSILON
            && ((self.y1 - self.y2).abs() - edge_size).abs() < std::f64::EPSILON
            && ((self.z1 - self.z2).abs() - edge_size).abs() < std::f64::EPSILON)
            || depth == max_depth
    }

    pub fn offset(mut self, offset: (f64, f64, f64)) -> Self {
        self.x1 += offset.0;
        self.y1 += offset.1;
        self.z1 += offset.2;
        self.x2 += offset.0;
        self.y2 += offset.1;
        self.z2 += offset.2;
        self
    }

    pub fn normalize(self) -> Self {
        let max_x = max(self.x1, self.x2);
        let max_y = max(self.x1, self.x2);
        let max_z = max(self.x1, self.x2);
        Self::new(
            self.x1 / max_x,
            self.y1 / max_y,
            self.z1 / max_z,
            self.x2 / max_x,
            self.y2 / max_y,
            self.z2 / max_z,
        )
    }

    pub fn normalize_with(self, normalization_vector: (f64, f64, f64)) -> Self {
        Self::new(
            self.x1 / normalization_vector.0,
            self.y1 / normalization_vector.1,
            self.z1 / normalization_vector.2,
            self.x2 / normalization_vector.0,
            self.y2 / normalization_vector.1,
            self.z2 / normalization_vector.2,
        )
    }
}
