#[cfg(test)]
mod tests {
    use geometry::prelude::*;

    #[test]
    fn test_bounding_box_area() {
        let bounding_box = BoundingBox::new(0.0, 0.0, 1.0, 1.0);

        assert_eq!(bounding_box.area(), 1.0);
    }

    #[test]
    fn test_bounding_box_touches() {
        let bounding_box1 = BoundingBox::new(0.0, 0.0, 1.0, 1.0);
        let bounding_box2 = BoundingBox::new(0.5, 0.5, 1.5, 1.5);

        assert!(bounding_box1.touches(&bounding_box2));

        let bounding_box3 = BoundingBox::new(2.0, 2.0, 3.0, 3.0);

        assert!(!bounding_box1.touches(&bounding_box3));
    }

    #[test]
    fn test_bounding_box_display() {
        let bounding_box = BoundingBox::new(0.0, 0.0, 1.0, 1.0);

        assert_eq!(format!("{}", bounding_box), "BoundingBox (0, 0, 1, 1)");
    }
}
