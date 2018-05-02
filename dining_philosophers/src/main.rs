struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.toString();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
