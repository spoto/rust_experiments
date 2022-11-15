#[cfg(test)]
use std::num::ParseIntError;

mod spores;
pub mod plant_structures;
pub mod another;
mod tests;

/// The starting point of the executable
fn main() {
    println!("Hello, world!");
}

#[allow(non_camel_case_types)]
pub struct git_revspec {
}

#[test]
#[should_panic(expected="divide by zero")]
#[allow(unused_must_use, unconditional_panic)]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[test]
fn convert() -> Result<(), ParseIntError> {
    i32::from_str_radix("1B2a", 14)?;
    Ok(())
}