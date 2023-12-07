use crate::BoundingBox;
#[derive(Debug)]
enum Hierarchy<T> {
    Leaf(T),
    Parent {
        left_child: Node<T>,
        right_child: Node<T>,
    },
}

#[derive(Debug)]
struct Node<T> {
    bounding_box: BoundingBox,
    hierarchy: Box<Hierarchy<T>>,
}

impl<T> Node<T> {
    fn insert(mut self, node: Node<T>) -> Node<T> {
        match *self.hierarchy {
            Hierarchy::Leaf(_) => Node {
                bounding_box: self.bounding_box.merge(&node.bounding_box),
                hierarchy: Box::new(Hierarchy::Parent {
                    left_child: self,
                    right_child: node,
                }),
            },
            Hierarchy::Parent {
                left_child,
                right_child,
            } => {
                self.bounding_box = self.bounding_box.merge(&node.bounding_box);

                let left_area = left_child.bounding_box.merge(&node.bounding_box).area();
                let right_area = right_child.bounding_box.merge(&node.bounding_box).area();

                self.hierarchy = Box::new(if left_area <= right_area {
                    Hierarchy::Parent {
                        left_child: left_child.insert(node),
                        right_child,
                    }
                } else {
                    Hierarchy::Parent {
                        left_child,
                        right_child: right_child.insert(node),
                    }
                });

                self
            }
        }
    }
}

pub struct BoundingBoxHierarchy<T> {
    root: Option<Node<T>>,
}

impl<T> BoundingBoxHierarchy<T> {
    pub fn new() -> Self {
        BoundingBoxHierarchy { root: None }
    }

    pub fn insert(&mut self, bounding_box: BoundingBox, value: T) {
        let node = Node {
            bounding_box,
            hierarchy: Box::new(Hierarchy::Leaf(value)),
        };

        self.root = match self.root.take() {
            Some(root) => Some(root.insert(node)),
            None => Some(node),
        };
    }

    pub fn get_touched_values(&self, bounding_box: &BoundingBox) -> Vec<&T> {
        let mut touched_values = Vec::new();

        if let Some(root) = &self.root {
            let mut stack = Vec::new();
            stack.push(root);

            while let Some(node) = stack.pop() {
                match *node.hierarchy {
                    Hierarchy::Leaf(ref value) => {
                        if bounding_box.touches(&node.bounding_box) {
                            touched_values.push(value);
                        }
                    }
                    Hierarchy::Parent {
                        ref left_child,
                        ref right_child,
                    } => {
                        if bounding_box.touches(&node.bounding_box) {
                            stack.push(&left_child);
                            stack.push(&right_child);
                        }
                    }
                }
            }
        }

        touched_values
    }
}
