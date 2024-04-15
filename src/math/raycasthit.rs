use crate::Vector;

#[derive(Debug, Clone, Copy)]
pub struct RayCastHit {
    pub hit: Option<(Vector, f32)>,
    pub normal: Option<Vector>,
    pub distance: f32,
    pub pos_on_screen: (i32, i32),
    pub uv: Option<(f32, f32)>,
}

impl RayCastHit {
    pub fn new(hit: Option<(Vector, f32)>) -> RayCastHit {
        RayCastHit {
            hit,
            normal: None,
            distance: 0.0,
            pos_on_screen: (0, 0),
            uv: None,
        }
    }

    pub fn with_normal(mut self, normal: Vector) -> RayCastHit {
        self.normal = Some(normal);
        self
    }

    pub fn with_distance(mut self, distance: f32) -> RayCastHit {
        self.distance = distance;
        self
    }

    pub fn with_uv(mut self, uv: (f32, f32)) -> RayCastHit {
        self.uv = Some(uv);
        self
    }

    pub fn is_some(&self) -> bool {
        self.hit.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.hit.is_none()
    }

    pub fn unwrap(&self) -> (Vector, f32) {
        self.hit.unwrap()
    }

    pub fn angle(&self) -> f32 {
        self.hit.unwrap().1
    }
}