mod clock;

use std::{thread, time};

fn main() {
    let mut clock = clock::Clock::new(900, 300);
    let one_sec = time::Duration::from_secs(1);

    loop {
        print!("\x1B[2J\x1B[1;1H");
        clock = clock.decrement();
        println!("{}",clock.show());
        thread::sleep(one_sec);
    }
}
