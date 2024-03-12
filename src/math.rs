// convert degress to radians
pub fn as_radians(angle: f64) -> f64 {
    angle * std::f64::consts::PI / 180.0
}

// convert radians to degrees
pub fn as_degrees(angle: f64) -> f64 {
    angle * 180.0 / std::f64::consts::PI
}






#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_radians() {
        assert_eq!(as_radians(180.0), std::f64::consts::PI);
    }

    #[test]
    fn test_as_degrees() {
        assert_eq!(as_degrees(std::f64::consts::PI), 180.0);
    }
}