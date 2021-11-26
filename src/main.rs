mod clock;

fn main() {
    let mut clock = clock::Clock::new(900, 300);
    for _ in 1..65 {
        clock.decrement();
    }
    println!("{}", clock.show())
}
