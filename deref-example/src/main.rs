use std::ops::Deref;

fn main() {
    let x = 5;
    let y1 = &x;
    let y2 = Box::new(x);
    let y3 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);
    assert_eq!(5, *y2);
    assert_eq!(5, *y3);


    // Passing a string slice.
    hello("Slice Rust");

    let m = MyBox::new(String::from("String Rust"));

    // (*m): Dereferences MyBox<String> into String.
    // &[..]: Takes a string slice of the String.
    hello(&(*m)[..]);

    // Syntactic sugar thanks to deref coercion.
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
