use jni::objects::AutoElements;

pub fn search_multiple(
    sorted_arr: AutoElements<i32>,
    numbers_to_find: AutoElements<i32>,
) -> Vec<i32> {
    let tree = BinarySearchTree::new(sorted_arr);
    numbers_to_find
        .iter()
        .map(|n| tree.search(n).unwrap_or(-1))
        .collect()
}

struct NodeData {
    pub data: i32,
    pub left: Box<Node>,
    pub rigth: Box<Node>,
    pub index: i32,
}

enum Node {
    Data(NodeData),
    None,
}

struct BinarySearchTree {
    root: Box<Node>,
}

impl BinarySearchTree {
    fn new(sorted_arr: AutoElements<i32>) -> Self {
        let root = BinarySearchTree::create_node(&sorted_arr, 0, (sorted_arr.len() - 1) as isize);
        Self {
            root: Box::new(root),
        }
    }

    pub fn search(&self, value: &i32) -> Option<i32> {
        BinarySearchTree::find_value(&self.root, value)
    }

    fn find_value(node: &Node, value: &i32) -> Option<i32> {
        match node {
            Node::Data(node) if &node.data == value => Some(node.index),
            Node::Data(node) if &node.data < value => {
                BinarySearchTree::find_value(&node.rigth, value)
            }
            Node::Data(node) => BinarySearchTree::find_value(&node.left, value),
            Node::None => None,
        }
    }

    fn create_node(sorted_arr: &AutoElements<i32>, left: isize, right: isize) -> Node {
        if left > right {
            return Node::None;
        }
        let middle = left + (right - left) / 2;
        let index = middle as usize;
        let data = sorted_arr[index];
        Node::Data(NodeData {
            data,
            index: index as i32,
            left: Box::new(BinarySearchTree::create_node(sorted_arr, left, middle - 1)),
            rigth: Box::new(BinarySearchTree::create_node(sorted_arr, middle + 1, right)),
        })
    }
}
