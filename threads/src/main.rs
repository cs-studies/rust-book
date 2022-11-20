use std::thread;
use std::time::Duration;

fn main() {
    play_spawn();
    play_move();
}

fn play_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Force the spawned thread to finish first.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("Main thread number {}", i);
        // println!("Is spawned thread finished: {}", handle.is_finished());
        thread::sleep(Duration::from_millis(1));
    }

    // println!("Is spawned thread finished: {}", handle.is_finished());
    handle.join().unwrap();
}

fn play_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
