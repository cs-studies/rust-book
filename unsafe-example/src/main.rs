fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("num is {num}");
    println!("r1 is {:?}", r1);
    println!("r2 is {:?}", r2);

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        danger();
    }

    // Uncertain validity.
    // let address = 0x12345usize;
    // let r3 = address as *const i32;
    // println!("r3 is {:?}", r3);

    let mut v = vec![10, 20, 30, 40, 50, 60];
    // let r = &mut v[..];
    let (a, b) = v.split_at_mut(3);

    assert_eq!(a, &mut [10, 20, 30]);
    assert_eq!(b, &mut [40, 50, 60]);

    unsafe {
        println!("Absolute value of -3 in C is {}", abs(-3));
        println!("Wow effect achieved.");
    }

    println!("The static value is '{}'", NO_WAY);

    unsafe {
        println!("COUNTER is {}", COUNTER);
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER is {} now", COUNTER);
    }
}

unsafe fn danger() {
    println!("A danger was called!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static NO_WAY: &str = "Yes, really.";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
