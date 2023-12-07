#[cfg(test)]
mod tests {
    use geometry::Vector;

    #[test]
    fn test_add() {
        let v1 = Vector(1.0, 2.0);
        let v2 = Vector(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vector(4.0, 6.0));
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector(1.0, 2.0);
        let v2 = Vector(3.0, 4.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, Vector(-2.0, -2.0));
    }

    #[test]
    fn test_scale() {
        let v = Vector(1.0, 2.0);
        let result = v.scale(3.0);
        assert_eq!(result, Vector(3.0, 6.0));
    }

    #[test]
    fn test_multiply() {
        let v1 = Vector(1.0, 2.0);
        let v2 = Vector(3.0, 4.0);
        let result = v1.multiply(&v2);
        assert_eq!(result, Vector(-5.0, 10.0));
    }

    #[test]
    fn test_square_magnitude() {
        let v = Vector(3.0, 4.0);
        let result = v.square_magnitude();
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector(3.0, 4.0);
        let result = v.magnitude();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_square_distance() {
        let v1 = Vector(1.0, 2.0);
        let v2 = Vector(4.0, 6.0);
        let result = v1.square_distance(&v2);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_distance() {
        let v1 = Vector(5.0, 10.0);
        let v2 = Vector(15.0, 20.0);
        let result = v1.distance(&v2);
        assert_eq!(result, 200.0f32.sqrt());
    }

    #[test]
    fn test_close_to() {
        let v1 = Vector(1.0, 2.0);
        let v2 = Vector(4.0, 6.0);
        assert!(!v1.close_to(&v2, 4.5));
        assert!(v1.close_to(&v2, 5.0));
    }

    #[test]
    fn test_display() {
        let v = Vector(3.0, 4.0);
        assert_eq!(format!("{}", v), "Vector (3, 4)");
    }
}
