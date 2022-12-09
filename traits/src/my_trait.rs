trait Super {
    fn do_it(&self) -> String;
    fn do_it_often(&self, n: u32) -> String {
        let mut s = "".to_string();
        for _ in 1 ..= n {
            s.insert_str(0, self.do_it().as_str());
        };
        s
    }
}

trait Sub where Self: Super {
    fn do_it_often(&self, n: u32) -> String;
}

impl Sub for i32 {
    fn do_it_often(&self, _: u32) -> String {
        "".to_string()
    }
}

impl Super for i32 {
    fn do_it(&self) -> String {
        use std::io::Write;
        let mut s = Vec::new();
        write!(&mut s, "{}", self).expect("error");
        String::from_utf8(s).expect("error")
    }
}

#[test]
fn test_medium() {
    let x = 13;
    assert_eq!(Super::do_it_often(&x, 17), "1313131313131313131313131313131313".to_string());
    assert_eq!(Sub::do_it_often(&x, 17), "".to_string());
}
