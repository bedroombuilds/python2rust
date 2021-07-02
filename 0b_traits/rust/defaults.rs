use std::default::Default;

#[derive(Debug, PartialEq, Default)]
struct D {
    x: i32,
    y: i32,
    z: i32,
}

impl D {
    fn new(x: i32, y: i32, z: Option<i32>) -> Self {
        match z {
            Some(z) => Self { x, y, z },
            None => Self { x, y, z: 0 },
        }
    }
}

impl std::ops::Add for D {
    type Output = D;

    fn add(self, other: D) -> D {
        D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let d1 = D::new(1, 2, None);
    let d2 = D {
        x: 1,
        y: 2,
        ..D::default()
    };
    assert_eq!(d1, d2);
    println!("{:?} {:?}", d1, d2);
    println!("{:?}", d1 + d2);
}
