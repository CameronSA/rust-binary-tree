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
