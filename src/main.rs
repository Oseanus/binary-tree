
#[derive(Clone, Copy)]
enum Order {
    Pre,
    In,
    Post
}

// The box type is working as a smart pointer. It is used to allow recursion.
#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Box<Node >>,
    right: Option<Box<Node>>
}

impl  Node {
    pub fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, data: i32) {
        // Return if value is already in tree.
        if self.data == data {
            return;
        }

        let target_node = if data < self.data {&mut self.left} else {&mut self.right};

        match target_node {
            None => {
                let new_node = Box::new(Node::new(data));
                *target_node = Some(new_node);
            },
            Some(node) => {
                node.insert(data)
            }
        }
    }

    pub fn traverse(&self, order: Order) {
        if let Order::Pre = order {
            println!("{:?}", self.data);
        }
        if let Some(l) = &self.left {
            l.traverse(order);
        }
        if let Order::In = order {
            println!("{:?}", self.data);
        }
        if let Some(r) = &self.right {
            r.traverse(order);
        }
        if let Order::Post = order {
            println!("{:?}", self.data);
        }
    }

    // pub fn swap(&mut self) {
        // let temp: mut Option<Box<Node>>;
        // *temp = Some(self.left);
        // self.left = self.right;
        // self.right = temp
    // }
}

fn main() {
    let mut  tree = Node::new(42);
    tree.insert(15);
    tree.insert(16);
    tree.insert(17);
    tree.insert(14);
    tree.insert(9);
    tree.insert(13);
    tree.insert(18);

    tree.traverse(Order::Pre);
    // tree.traverse(Order::In);
    // tree.traverse(Order::Post);
}
