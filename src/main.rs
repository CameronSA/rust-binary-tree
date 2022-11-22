use rust_binary_tree::Node;

fn main() {
    let mut root = Node::new(1);
    let mut left = Node::new(2);
    let right = Node::new(3);

    left.add_left(Node::new(4));
    left.add_right(Node::new(5));

    root.add_left(left);
    root.add_right(right);

    println!("{:?}", root);
}
