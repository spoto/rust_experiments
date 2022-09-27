fn main() {
    println!("Run the tests in this program: \"cargo test\" or \"cargo test --release\"")
}

#[test]
fn string_tuple() {
    let text = "I see the eigenvalue in thine eyes";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eyes");
}

// in debug mode, this panics; in release mode, computes -32768...
#[test]
#[should_panic]
fn print_abs() {
    let i:i16 = -32768;
    println!("abs({}) = {}", i, i.abs());
}

#[test]
#[should_panic]
// attempts to forge chars from illegal UNICODE code points are caught at run time
fn print_chars() {
    let mut c = '\u{d700}';
    for _offset in 0..1000 {
        println!("UNICODE {}: {}", c as u32, c);
        c = char::from_u32(c as u32 + 1).expect("out of UNICODE range");
    }
}

// this panics in debug mode and diverges in release mode!
#[test]
#[should_panic]
fn infinite() {
    let mut _i = 1;
    loop {
        _i *= 10;
    }
}

#[test]
#[should_panic]
// this panics in any case; corresponds to the behaviour in a debug build
fn checked_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

#[allow(dead_code)]
// this diverges in any case: corresponds to the behaviour in a release build
fn wrapping_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.wrapping_mul(10);
    }
}

#[test]
fn test_tuples() {
    let t1 = (13, 17);
    let t2 = (13, 17);
    assert_eq!(t1, t2);
}

#[test]
fn test_assign_arrays() {
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1; // makes a copy of arr1
    assert_eq!(arr1, arr2);
}

#[test]
fn test_assign_modify_arrays() {
    let arr1 = [1, 2, 3, 4];
    let mut arr2 = arr1; // makes a copy of arr1
    arr2[2] = 5;
    assert_ne!(arr1, arr2);
}

#[test]
fn test_assign_tuples() {
    let t1 = (1, 2);
    let t2 = t1; // makes a copy of t1
    assert_eq!(t1, t2);
}

#[test]
fn test_assign_modify_tuples() {
    let t1 = (1, 2);
    let mut t2 = t1; // makes a copy of t1
    t2.1 = 5;
    assert_ne!(t1, t2);
}

// the following does not compile since vectors get moved
/*#[test]
fn test_assign_vectors() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = v1; // makes a copy of v1
    assert_eq!(v1, v2);
}*/

// the following does not compile since vectors get moved
/*#[test]
fn test_assign_modify_vectors() {
    let v1 = vec![1, 2, 3, 4];
    let mut v2 = v1; // makes a copy of v1
    v2[2] = 5;
    assert_ne!(v1, v2);
}*/

#[test]
fn vec_product() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
}

#[test]
fn vec_len_capacity() {
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());
}

fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for x in slice {
        sum += x;
    };

    sum
}

#[test]
fn test_equal_slices() {
    let v = vec![1, 5, -2, 12, 35];
    let a = [1, 5, -2, 12, 35];
    let sv : &[i16] = &v;
    let sa: &[i16] = &a;
    assert_eq!(sv, sa);
}

#[test]
fn test_not_equal_slices() {
    let v = vec![1, 5, -2, 12, 35];
    let a = [1, 5, -2, 12, 36];
    let sv : &[i16] = &v;
    let sa: &[i16] = &a;
    assert_ne!(sv, sa);
}

#[test]
fn test_sum_slices() {
    let v = vec![1, 5, -2, 12, 35];
    let a = [1, 5, -2, 12, 35];
    let sv : &[i32] = &v;
    let sa: &[i32] = &a;
    assert_eq!(sum(sv), 51);
    assert_eq!(sum(sa), 51);
    assert_eq!(sum(&sv[2..4]), 10);
    assert_eq!(sum(&sa[2..]), 45);
}

#[test]
fn test_string_literals() {
    let _s1 = "ciao";
    let s2 = b"ciao";
    assert_eq!(b'c', s2[0]);
}

#[test]
fn test_stirs() {
    let s1 = "noodles";
    let mut s2 = s1.to_string();
    let s3= &mut s2[..];
    s3.make_ascii_uppercase();
    assert_eq!("noodles", s1);
    assert_eq!("NOODLES", s2);
}

#[test]
fn test_stir_vector() {
    let bits = vec!["veni", "vidi", "vici"];
    let c = bits.concat();
    let j = bits.join(", ");
    assert_eq!(c, "venividivici");
    assert_eq!(j, "veni, vidi, vici");
}