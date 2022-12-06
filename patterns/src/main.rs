fn main() {
    play_match();
    play_iflet();
    play_whilelet();
    play_for();
    play_let();

    play_params(&(42, 43));

    play_match_variations();
    play_struct_destruct();
}

fn play_match() {
    println!("** Play `match`");

    let mut x = Some(1);
    match x {
        Some(i) => {
            x = Some(i + 1);
        }
        None => {
            x = Some(1);
        }
    }
    println!("x = {:?}", x);
}

fn play_iflet() {
    println!("\n** Play `if let`");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "42".parse();

    if let Some(color) = favorite_color {
        println!("Using favorite_color {color}");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple because the shadowed age is {age}");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Using blue");
    }
}

fn play_whilelet() {
    println!("\n** Play `while let`");

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn play_for() {
    println!("\n** Play `for`");

    let v = vec!['h', 'e', 'l', 'l', 'o'];

    for (index, key) in v.iter().enumerate() {
        println!("{} is at index {}", key, index);
    }
}

fn play_let() {
    println!("\n** Play `let`");

    let (x, y, z) = (5, 7, 3);

    println!("x = {x}");
    println!("x = {y}");
    println!("x = {z}");
}

fn play_params(&(x, y): &(i32, i32)) {
    println!("\n** Play function params");
    println!("x = {x}, y = {y}");
}

fn play_match_variations() {
    println!("\n** Play `match` variations");
    println!("\n*** `match` a literal");
    let y = 3;
    match y {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("\n*** `match` a named variable");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, inner scope y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, outer scope y = {y}", x);

    println!("\n*** `match` multiple patterns");
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("\n*** `match` ranges");
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn play_struct_destruct() {
    println!("\n** Play struct destruction");

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    println!("x = {x}, y = {y}");
    // Shorthand for
    // let Point { x: x, y: y } = p;
    // let Point { x: a, y: b } = p;
    // println!("a = {a}, b = {b}");
}
