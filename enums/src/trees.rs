use std::mem::{size_of, size_of_val};

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

fn select_root<T>(t: TreeNode<T>) -> T {
    match t {
        TreeNode { element, .. } => element
    }
}

fn select_root2<T>(TreeNode { element, .. }: TreeNode<T>) -> T {
    element
}

#[test]
fn test_select_root() {
    let lt = BinaryTree::Empty;
    let rt = BinaryTree::Empty;
    let bt = TreeNode{ element: 13, left: lt, right: rt};
    assert_eq!(select_root(bt), 13);
}

#[test]
fn test_select_root2() {
    let lt = BinaryTree::Empty;
    let rt = BinaryTree::Empty;
    let bt = TreeNode{ element: 13, left: lt, right: rt};
    assert_eq!(select_root2(bt), 13);
}

#[test]
fn test_tree() {
    let lt = BinaryTree::Empty;
    let rt = BinaryTree::Empty;
    let bt = BinaryTree::NonEmpty(Box::new(TreeNode{ element: 13, left: lt, right: rt}));
    let size2 = size_of_val(&bt);
    let size3 = size_of::<BinaryTree<i32>>();
    let size4 = size_of::<TreeNode<i32>>();
    assert_eq!(size2, 8);
    assert_eq!(size3, 8);
    assert_eq!(size4, 24);
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T> BinaryTree<T> {
    fn size(&self) -> usize {
        match *self {
            BinaryTree::Empty => 0,
            BinaryTree::NonEmpty(ref node) => 1 + node.left.size() + node.right.size()
        }
    }
}

#[test]
fn test_add() {
    let mut bt = BinaryTree::Empty;
    bt.add("Mars");
    bt.add("Venus");
    bt.add("Jupiter");
    bt.add("Earth");
    bt.add("Uranus");
    bt.add("Saturn");
    assert_eq!(bt.size(), 6);
}