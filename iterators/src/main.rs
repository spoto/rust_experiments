use std::fmt::Debug;

mod ranges;
mod vector;
mod str;
mod pipeline;
mod narcissistic;
mod terminal;

fn main() {
    println!("Hello, world!");
    dump("Hello, world!".chars());
}

trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait MyIntoIterator {
    type Item;
    type IntoIter: MyIterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

fn dump<T, U>(t: T)
    where T: IntoIterator<Item=U>,
          U: Debug
{
    for u in t {
        println!("{:?}", u);
    }
}