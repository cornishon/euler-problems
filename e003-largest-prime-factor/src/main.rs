use std::collections::HashMap;

fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut factors = HashMap::<u64, u64>::new();
    let maxn = f64::sqrt(n as f64) as u64 + 2;
    let mut m = n;

    for k in 2..maxn {
        while m % k == 0 {
            let count = factors.entry(k).or_insert(0);
            *count += 1;
            m /= k;
        }
    }

    factors
}

fn main() {
    let n = 600851475143;
    let factors = prime_factors(n);
    let max_factor = factors.keys().max().unwrap();

    println!("{}", &max_factor);

}
