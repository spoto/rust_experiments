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

#[test]
fn test_tuple_struct() {
    struct Bounds(usize, usize);
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}

#[test]
fn test_unit_like_struct() {

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Onesuch;

    let os = Onesuch;
    assert_eq!(os, os);
}

#[test]
fn test_queue() {
    let mut q = Queue::new();

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Queue {
        older: Vec<char>,
        younger: Vec<char>
    }

    q.push('h');
    q.push('a');
    q.push('e');
    assert_eq!(q.pop(), Some('h'));
    assert_eq!(q.pop(), Some('a'));
    assert_eq!(q.pop(), Some('e'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());

    q.push('S');
    q.push('P');
    q.pop();
    q.push('Q');
    q.push('R');
    let (older, younger) = q.split();
    assert_eq!(older, vec!['P']);
    assert_eq!(younger, vec!['Q', 'R']);

    let mut bq = Box::new(Queue::new());
    bq.is_empty();
    bq.push('A');
    bq.split();

    let mut q1 = Queue::EMPTY;
    q1.push('a');
    let mut q2 = Queue::EMPTY;
    assert!(q2.is_empty());
    assert!(!q1.is_empty());
    assert_eq!(Queue::EMPTY, Queue::EMPTY);

    impl Queue {
        pub const fn new() -> Queue {
            Queue{ older: Vec::new(), younger: Vec::new() }
        }

        const EMPTY: Queue = Queue::new();

        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }

        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }

                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }

            self.older.pop()
        }

        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }

        pub fn split(self) -> (Vec<char>, Vec<char>) {
            let o = self.older;
            let y = self.younger;
            (o, y)
        }
    }
}

#[test]
fn test_queue_generic() {
    let mut q = Queue::new(); // ok, type inference
    //let mut q = Queue::<String>::new(); // ok

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>
    }

    q.push("h".to_string());
    q.push("a".to_string());
    q.push("e".to_string());
    assert_eq!(q.pop(), Some("h".to_string()));
    assert_eq!(q.pop(), Some("a".to_string()));
    assert_eq!(q.pop(), Some("e".to_string()));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());

    impl<T> Queue<T> {
        pub const fn new() -> Self {
            // Queue::<T>{ older: Vec::new(), younger: Vec::new() } // ok
            // Queue{ older: Vec::new(), younger: Vec::new() } // ok, type inference
            Self{ older: Vec::new(), younger: Vec::new() }
        }

        const EMPTY: Self = Queue::new();

        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }

                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }

            self.older.pop()
        }

        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }

        pub fn split(self) -> (Vec<T>, Vec<T>) {
            let o = self.older;
            let y = self.younger;
            (o, y)
        }
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = greatest;

    for x in slice {
        if *x < *least {
            least = x;
        }
        else if *x > *greatest {
            greatest = x;
        }
    }

    Extrema { greatest, least }
}

#[test]
fn test_lifetime_pars() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

#[test]
fn test_derived_traits() {

    #[derive(Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64
    }

    let mut p1 = Point { x: 3.14, y: 2.73 };
    let mut p2 = Point { x: 3.14, y: 2.73 };

    assert_eq!(p1, p2);

    p1 = p2;

    assert_eq!(p1.x, 3.14);
}

#[test]
fn test_ref_cell() {
    use std::cell::RefCell;

    let ref_cell : RefCell<String> = RefCell::new("hello".to_string());

    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    } // this will drop r

    {
        let mut w = ref_cell.borrow_mut();
        w.push_str(" world");
        assert_eq!(w.len(), 11);
    }
}