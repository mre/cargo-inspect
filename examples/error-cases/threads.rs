/// Comments in threads won't appear at the expected position
/// See for yourself by running `cargo inspect` on this file.

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        // Above for loop
        for i in 1..10 {
            // Above println
            println!("hi number {} from the spawned thread!", i);

            // Above thread::sleep
            thread::sleep(Duration::from_millis(1));
        }
    });
}
