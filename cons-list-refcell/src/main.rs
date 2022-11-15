#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("BEFORE");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    *value.borrow_mut() += 10;

    println!("AFTER");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
