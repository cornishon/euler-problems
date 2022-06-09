struct Fib {
  value: u64,
    prev: u64,
}

impl Fib {
    fn new() -> Fib {
        Fib {
            value: 1,
            prev: 0,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_fib = self.value + self.prev;
        self.prev = self.value;
        self.value = next_fib;
        Some(next_fib)
    }
}

fn main() {
    let sum: u64 = Fib::new()
        .take_while(|x| *x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();

  println!("{}", sum);
}
