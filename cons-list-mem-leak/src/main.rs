use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("INITIAL a = {:?}", a);
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("INITIAL b = {:?}", b);
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("AFTER CHANGING a");
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));

    // Overflows the stack.
    // Demonstrates a cycle that causes a memory leak.
    // println!("a next item = {:?}", a.tail());
}
