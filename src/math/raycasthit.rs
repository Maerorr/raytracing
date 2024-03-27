use crate::Vector;

#[derive(Debug, Clone, Copy)]
pub struct RayCastHit {
    pub hit: Option<(Vector, f64)>,
    pub normal: Option<Vector>,
    pub pos_on_screen: (i32, i32),
}

impl RayCastHit {
    pub fn new(hit: Option<(Vector, f64)>) -> RayCastHit {
        RayCastHit {
            hit,
            normal: None,
            pos_on_screen: (0, 0),
        }
    }

    pub fn with_normal(mut self, normal: Vector) -> RayCastHit {
        self.normal = Some(normal);
        self
    }

    pub fn is_some(&self) -> bool {
        self.hit.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.hit.is_none()
    }

    pub fn unwrap(&self) -> (Vector, f64) {
        self.hit.unwrap()
    }

    pub fn angle(&self) -> f64 {
        self.hit.unwrap().1
    }
}