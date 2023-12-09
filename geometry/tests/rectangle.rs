#[cfg(test)]
mod tests {
    use geometry::Rectangle;
    use geometry::Vector;

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 10.0,
            center: Vector(4000.8, 9214.14),
        };
        assert_eq!(rectangle.area(), 50.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 10.0,
            center: Vector(9.0, -99.9999),
        };
        assert_eq!(rectangle.perimeter(), 30.0);
    }

    #[test]
    fn test_rectangle_contains_point() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 10.0,
            center: Vector(2.0, 1.0),
        };
        let point_inside = Vector(2.0, 3.0);
        let point_outside = Vector(6.0, 12.0);
        assert!(rectangle.contains(point_inside));
        assert!(!rectangle.contains(point_outside));
    }

    #[test]
    fn test_rectangle_display() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 10.0,
            center: Vector(2.0, 1.0),
        };
        assert_eq!(format!("{}", rectangle), "Rectangle 5x10 (2, 1)");
    }
}
