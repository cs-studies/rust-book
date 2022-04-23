fn main() {
    println!("");
    basics();

    println!("");
    ternany();

    println!("");
    count_up();

    println!("");
    counter();

    println!("");
    liftoff();

    println!("");
    play_for();
}

fn basics() {
    let number = 3;

    if number < 5 {
        println!("basics speaking: condition was true");
    } else {
        println!("basics speaking: condition was false");
    }
}

fn ternany() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("ternany speaking: The value of number is: {}", number);
}

fn count_up() {
    let mut count = 0;
    'counting_up: loop {
        println!("count_up speaking: count = {}", count);
        let mut remaining = 10;

        loop {
            println!("count_up speaking: remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("count_up speaking: End count = {}", count);
}

fn counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("counter speaking: The result is {}", result);
}

fn liftoff() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn play_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("play_for speaking: the value is: {}", element);
    }
}
