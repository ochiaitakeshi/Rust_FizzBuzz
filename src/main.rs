use std::thread;
use std::time::Duration;

// FizzBuzz(スレッドを使った不完全版)

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(30));
        for i in 1..101 {
            if i % 15 == 0 {
                // FizzBuzzの場合
                print!("FizzBuzz ")
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        for i in 1..101 {
            if (i % 3 == 0) && (i % 15 != 0) {
                // Fizz単独の場合
                print!("Fizz ")
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(20));
        for i in 1..101 {
            if (i % 5 == 0) && (i % 15 != 0) {
                print!("Buzz ")
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..101 {
        if !( (i % 3 == 0) || (i % 5 == 0) ){
            print!("{} ", i)
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}
