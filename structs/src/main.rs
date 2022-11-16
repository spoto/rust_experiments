mod broom;

struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn main() {}

#[test]
fn test_create_struct() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height)
    };
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

#[test]
fn test_create_struct_same_name() {
    let width = 1024;
    let height = 576;
    let image = new_map((width, height), vec![0; width * height]);
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}