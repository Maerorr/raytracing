// convert degress to radians
pub fn as_radians(angle: f32) -> f32 {
    angle * std::f32::consts::PI / 180.0
}

// convert radians to degrees
pub fn as_degrees(angle: f32) -> f32 {
    angle * 180.0 / std::f32::consts::PI
}






#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_radians() {
        assert_eq!(as_radians(180.0), std::f32::consts::PI);
    }

    #[test]
    fn test_as_degrees() {
        assert_eq!(as_degrees(std::f32::consts::PI), 180.0);
    }
}