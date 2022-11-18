use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf rc strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // No cycle (no stack overflow) due to the weak reference.
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!("branch rc strong = {}, weak = {}",
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch),
        );

        println!("leaf rc strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf),
        );
    }

    println!("outer scope leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf rc strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf),
    );
}
