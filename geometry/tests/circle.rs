#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use geometry::Circle;
    use geometry::Vector;

    #[test]
    fn test_circle_area() {
        let circle = Circle {
            radius: 5.0,
            center: Vector(1.0, 2.0),
        };
        let expected_area = 78.53981633974483;
        assert_eq!(circle.area(), expected_area);
    }

    #[test]
    fn test_circle_circumference() {
        let circle = Circle {
            radius: 3.0,
            center: Vector(3.0, 4.0),
        };
        let expected_circumference = 6.0 * PI;
        assert_eq!(circle.circumference(), expected_circumference);
    }

    #[test]
    fn test_circle_contains() {
        let radius = 7.0;
        let circle = Circle {
            radius,
            center: Vector(3.0, 4.0),
        };
        assert!(circle.contains(Vector(1.0, 2.0)));
        assert!(circle.contains(Vector(3.0, 11.0)));
        assert!(!circle.contains(Vector(3.0, 11.5)));
        assert!(!circle.contains(Vector(8.0, 9.0)));
    }

    #[test]
    fn test_circle_display() {
        let circle = Circle {
            radius: 5.0,
            center: Vector(1.0, 2.0),
        };
        let expected_display = "Circle 5 (1, 2)";
        assert_eq!(format!("{}", circle), expected_display);
    }
}
