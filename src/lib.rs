use std::cmp::Ordering;

pub struct Node {
    key: i64,
    left_ptr: Option<Box<Node>>,
    right_ptr: Option<Box<Node>>,
}

#[derive(Default)]
pub struct Bst {
    root_ptr: Option<Box<Node>>,
}

impl Bst {
    pub fn insert(&mut self, key: i64) -> bool {
        let mut node_ptr = &mut self.root_ptr;
        while let Some(node) = node_ptr {
            match node.key.cmp(&key) {
                Ordering::Equal => return false,
                Ordering::Less => node_ptr = &mut node.right_ptr,
                Ordering::Greater => node_ptr = &mut node.left_ptr,
            }
        }

        *node_ptr = Some(Box::new(Node {
            key,
            left_ptr: None,
            right_ptr: None,
        }));
        true
    }

    pub fn contains_match(&self, key: i64) -> bool {
        let mut node_ptr = &self.root_ptr;
        while let Some(node) = node_ptr {
            match node.key.cmp(&key) {
                Ordering::Equal => return true,
                Ordering::Less => node_ptr = &node.right_ptr,
                Ordering::Greater => node_ptr = &node.left_ptr,
            }
        }
        false
    }

    pub fn contains_if_else(&self, key: i64) -> bool {
        let mut node_ptr = &self.root_ptr;
        while let Some(node) = node_ptr {
            if node.key == key {
                return true;
            }
            if node.key < key {
                node_ptr = &node.right_ptr;
            } else {
                node_ptr = &node.left_ptr;
            }
        }
        false
    }
}
