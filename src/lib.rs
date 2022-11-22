#[derive(Debug)]
pub struct Node {
    id: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(id: i32) -> Self {
        Node {
            id,
            left: None,
            right: None,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn left(&self) -> &Option<Box<Node>> {
        &self.left
    }

    pub fn right(&self) -> &Option<Box<Node>> {
        &self.right
    }

    pub fn add_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    pub fn add_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn test_add_nodes() {
        let mut node = Node::new(1);
        node.add_left(Node::new(2));
        node.add_right(Node::new(3));

        let expected = "Node { id: 1, left: Some(Node { id: 2, left: None, right: None }), right: Some(Node { id: 3, left: None, right: None }) }";

        let actual = format!("{:?}", node);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_nodes() {
        let mut node = Node::new(1);
        node.add_left(Node::new(2));
        node.add_right(Node::new(3));

        let left = match node.left() {
            Some(left) => left,
            None => panic!("left is None"),
        };

        let right = match node.right() {
            Some(right) => right,
            None => panic!("left is None"),
        };

        let expected_left = "Node { id: 2, left: None, right: None }";
        let expected_right = "Node { id: 3, left: None, right: None }";

        let actual_left = format!("{:?}", left);

        let actual_right = format!("{:?}", right);

        assert_eq!(expected_left, actual_left);
        assert_eq!(expected_right, actual_right);
    }

    #[test]
    fn test_get_id() {
        let node = Node::new(1);

        let id = node.id();

        assert_eq!(id, 1);
    }
}
