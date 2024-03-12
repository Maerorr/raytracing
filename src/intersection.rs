trait Intersection {
    fn intersect(&self, ray: &Ray) -> Option<RayCastHit>;
}

