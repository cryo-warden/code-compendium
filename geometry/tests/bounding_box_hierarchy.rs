#[cfg(test)]
mod tests {
    use geometry::{BoundingBox, BoundingBoxHierarchy};

    fn get_brute_force_touched_values(
        bounding_boxes: &Vec<BoundingBox>,
        query_box: &BoundingBox,
    ) -> Vec<usize> {
        bounding_boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| b.touches(query_box))
            .map(|(i, _)| i)
            .collect()
    }

    #[test]
    fn test_bounding_box_hierarchy() {
        let mut hierarchy = BoundingBoxHierarchy::new();

        let bounding_boxes = vec![
            BoundingBox::new(-10.0, -10.0, 0.0, 0.0),
            BoundingBox::new(0.0, 0.0, 10.0, 10.0),
            BoundingBox::new(5.0, 5.0, 15.0, 15.0),
            BoundingBox::new(10.0, 10.0, 20.0, 20.0),
            BoundingBox::new(15.0, 15.0, 25.0, 25.0),
            BoundingBox::new(20.0, 20.0, 30.0, 30.0),
            BoundingBox::new(25.0, 25.0, 35.0, 35.0),
        ];

        bounding_boxes
            .iter()
            .enumerate()
            .for_each(|(i, b)| hierarchy.insert(*b, i.clone()));

        let query_boxes = vec![
            BoundingBox::new(3.0, 3.0, 7.0, 7.0),
            BoundingBox::new(8.0, 8.0, 12.0, 12.0),
            BoundingBox::new(13.0, 13.0, 17.0, 17.0),
            BoundingBox::new(18.0, 18.0, 22.0, 22.0),
            BoundingBox::new(23.0, 23.0, 27.0, 27.0),
            BoundingBox::new(28.0, 28.0, 32.0, 32.0),
        ];

        for b in query_boxes {
            let result = hierarchy.get_touched_values(&b);
            let brute_result = get_brute_force_touched_values(&bounding_boxes, &b);

            println!("result: {:?}, {:?}, {:?}", b, result, brute_result);
            assert_eq!(result.len(), brute_result.len());

            let result_hashset = result.into_iter().collect::<std::collections::HashSet<_>>();

            for i in brute_result {
                assert!(result_hashset.contains(&i));
            }
        }
    }
}
