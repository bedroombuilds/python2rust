//! # most amazing geometric object library ever
//! This is starting off like clockwork
//! - bullet
//! - points

/// for surface area calculation of a geometric object
pub trait Area {
    fn area(&self) -> f64;
}

/// for Perimeter/circumferen calculation of a geometric object
pub trait Perimeter {
    fn circumference(&self) -> f64;
}

/// Circle object, coords and radius as properties
pub struct Circle {
    /// x-coordinate in 2D space
    pub x: f64,
    /// y-coordinate in 2D space
    pub y: f64,
    /// Radius, unit-less
    pub radius: f64,
}

/// Circle object supports area computation
impl Area for Circle {
    /// most efficient way to compute area
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius.powi(2))
    }
}

/// Circle also supports circumference
impl Perimeter for Circle {
    /// most efficient way to compute perimeter
    fn circumference(&self) -> f64 {
        std::f64::consts::PI * 2. * self.radius
    }
}

/// Square object, coords and sidelength as properties
pub struct Square {
    /// x-coordinate in 2D space
    pub x: f64,
    /// y-coordinate in 2D space
    pub y: f64,
    /// sidelength unit-less
    pub sidelen: f64,
}

/// Only Area supported, circumference is too hard to compute
impl Area for Square {
    /// super amazing surface area computation
    /// ```
    /// # use geo::Area;
    /// let s = geo::Square{x: 1.0, y: 3.0, sidelen: 2.0};
    /// assert_eq!(s.area(), 4.);
    /// ```
    fn area(&self) -> f64 {
        self.sidelen.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let c = Circle {
            x: 0.0,
            y: 1.0,
            radius: 3.0,
        };
        assert_eq!(c.area(), 28.274333882308138);
    }
}
