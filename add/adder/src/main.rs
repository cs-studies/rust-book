use add_one;
use add_two;
use rand;

fn main() {
    let mut num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    num = 40;
    println!("Hey, but {num} plus two is {}!", add_two::add_two(num));
}
