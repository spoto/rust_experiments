use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[test]
fn test_index() {
    let mut m = HashMap::new();
    m.insert("è", 13);
    m.insert("é", 17);
    let x = m["è"];
    assert_eq!(x, 13);
    let y = *(m.index("è"));
    assert_eq!(x, y);
    assert_ne!(x, m["é"]);
}

#[test]
fn test_mut_index() {
    let mut m = HashMap::new();
    m.insert("è", 13);
    m.insert("é", 17);
    // m["e"] = 42; // doesn't work since HashMap doesn't implement IndexMut
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P> Image<P> {
    fn new(width: usize, height: usize) -> Image<P>
        where P: Default + Copy
    {
        Image {
            width,
            pixels: vec![P::default(); width * height]
        }
    }
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..(start + self.width)]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..(start + self.width)]
    }
}

#[test]
fn test_image() {
    let mut image = Image::<u8>::new(13, 17);
    image[3][7] = 42;
    image[16][12] = image[3][7];
    assert_eq!(image[16][12], 42);
    assert_eq!(image[16][11], 0);
}