use geometry::{BoundingBox, BoundingBoxHierarchy, Circle, Rectangle, Vector};

fn main() {
    let mut a = Vector(5.0, 5.0);
    let b = Vector(2.0, 3.0);

    println!("{}", a);
    println!("{}", b);

    a += b;
    println!("{}", a);

    a *= b;
    println!("{}", a);

    a *= 2.0;
    println!("{}", a);

    a /= 2.0;
    println!("{}", a);

    a -= b;
    println!("{}", a);
    println!("{}", b);

    a -= Vector(1.0, 1.0);
    println!("{}", a.magnitude());
    println!("{}", a);
    println!("{}", b);

    let mut c = Circle {
        center: Vector(3.0, 2.0),
        radius: 5.0,
    };
    println!("{}", c);

    c.center += b;
    println!("{}", c.area());
    println!("{}", c);
    println!("{}", c.center.distance(Vector(2.0, 4.0)));
    println!("{}", c.contains(Vector(2.0, 4.0)));
    println!("{}", c.center.distance(Vector(8.0, 24.0)));
    println!("{}", c.contains(Vector(8.0, 24.0)));
    println!("{}", c);

    let mut r = Rectangle {
        center: Vector(0.0, 0.0),
        width: 10.0,
        height: 10.0,
    };

    println!("{}", r);

    r.center += Vector(3.0, 9.0);

    println!("{}", r);

    let bounding_boxes = vec![
        BoundingBox::new(-10.0, -10.0, 0.0, 0.0),
        BoundingBox::new(0.0, 0.0, 10.0, 10.0),
        BoundingBox::new(5.0, 5.0, 15.0, 15.0),
        BoundingBox::new(10.0, 10.0, 20.0, 20.0),
        BoundingBox::new(15.0, 15.0, 25.0, 25.0),
        BoundingBox::new(20.0, 20.0, 30.0, 30.0),
        BoundingBox::new(25.0, 25.0, 35.0, 35.0),
    ];

    let mut bounding_box_hierarchy = BoundingBoxHierarchy::new();
    for (i, bounding_box) in bounding_boxes.into_iter().enumerate() {
        bounding_box_hierarchy.insert(bounding_box, i);
    }

    let query_boxes = vec![
        BoundingBox::new(3.0, 3.0, 7.0, 7.0),
        BoundingBox::new(8.0, 8.0, 12.0, 12.0),
        BoundingBox::new(13.0, 13.0, 17.0, 17.0),
        BoundingBox::new(18.0, 18.0, 22.0, 22.0),
        BoundingBox::new(23.0, 23.0, 27.0, 27.0),
        BoundingBox::new(28.0, 28.0, 32.0, 32.0),
    ];

    for query_box in query_boxes {
        let result = bounding_box_hierarchy.get_touched_values(&query_box);
        println!("{:?}", result);
    }
}
