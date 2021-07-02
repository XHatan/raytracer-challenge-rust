
use crate::*;
use self::Kind::*;
use crate::shape_props::ShapeProperties;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum Kind {
    Solid(Tuple),
    Stripe(Tuple, Tuple),
    Gradient(Tuple, Tuple),
    Checkers(Tuple, Tuple),
    Ring(Tuple, Tuple),
}

fn alternate(t: i64, a: Tuple, b: Tuple) -> Tuple {
    match t % 2 == 0 {
        true => a,
        false => b
    }
}

impl Kind {
    fn color_at(&self, point: Point) -> Tuple {
        match self {
            Solid(colour) => colour.clone(),
            Stripe(a, b) => {
                let t = point.x().floor() as i64;
                let color_a = a.clone();
                let color_b = b.clone();
                alternate(t, color_a, color_b)
            },
            Gradient(a, b) => {
                let color_a = a.clone();
                let color_b = b.clone();
                let distance = color_b - color_a;
                let fraction = point.x() - point.x().floor();
                let df = distance * fraction;
                color_a + df
            }
            Ring(a, b) => {
                let color_a = a.clone();
                let color_b = b.clone();
                let t = (point.x() * point.x() + point.z() * point.z()).sqrt().floor() as i64;
                alternate(t, color_a, color_b)
            }
            Checkers(a, b) => {
                let color_a = a.clone();
                let color_b = b.clone();
                let t = (point.x().floor() + point.y().floor() + point.z().floor()) as i64;
                alternate(t, color_a, color_b)
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Pattern {
    pub kind: Kind,
    pub transform: Transform,
}

impl Pattern {
    pub fn set_transform(&mut self, transform: &Transform)  {
        self.transform = transform.clone();

    }
    pub fn color_at(&self, point: Point) -> Tuple {
        self.kind.color_at(point)
    }

    pub fn color_at_object(&self, shape: &dyn ShapeProperties, point: Point) -> Tuple {
        let object_point = shape.transform().inverse() * point;
        let pattern_point = self.transform.inverse() * object_point;
        self.kind.color_at(pattern_point)
    }
}


pub fn black_pattern() -> Pattern {
    Pattern {
        kind: Solid(Tuple::new(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::new()
    }
}

pub fn stripe_pattern(color_a: Tuple, color_b: Tuple) -> Pattern {
    Pattern {
        kind: Stripe(color_a, color_b),
        transform: Transform::new()
    }
}

pub fn gradient_pattern(a: Tuple, b: Tuple) -> Pattern {
    Pattern {
        kind: Gradient(a, b),
        transform: Transform::new()
    }
}

pub fn checkers_pattern(a: Tuple, b: Tuple) -> Pattern {
    Pattern {
        kind: Checkers(a, b),
        transform: Transform::new(),
    }
}

pub fn ring_pattern(a: Tuple, b: Tuple) -> Pattern {
    Pattern {
        kind: Ring(a, b),
        transform: Transform::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_equal() {
        let a = black_pattern();
        let b = black_pattern();
        assert_eq!(a == b, true);
    }
}