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