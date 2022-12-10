use num::Complex;

trait MyAdd<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd for i32 {
    type Output = i32;

    // fn add(self, rhs: i32) -> i32 // the same
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

trait OtherAdd<Rhs = Self, Output = Self> {
    fn add2(self, rhs: Rhs) -> Output;
}

impl OtherAdd for i32 {
    fn add2(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[test]
fn test_add() {
    let x = 42;
    assert_eq!(x.add(13), 55);
}

#[test]
fn test_add2() {
    let x = 42;
    assert_eq!(x.add2(13), 55);
}

impl<T: OtherAdd> OtherAdd for Complex<T> {
    fn add2(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re.add2(rhs.re),
            im: self.im.add2(rhs.im)
        }
    }
}

#[test]
fn test_add2_complex() {
    let c = Complex { re: 3, im: -5 };
    let a = c.add2(c);
    assert_eq!(a.re, 6);
    assert_eq!(a.im, -10);
}
