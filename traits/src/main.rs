use std::io::Write;

fn main() {
    println!("Hello, world!");
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

#[test]
fn test_say_hello() -> std::io::Result<()> {
    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");
    Ok(())
}