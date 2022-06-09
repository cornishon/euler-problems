fn main() {
    let range = 1..=100;
    let sum_squares: u64 = range.clone()
        .map(|x| x*x)
        .sum();

    let square_sum: u64 = range.clone()
        .sum::<u64>()
        .pow(2);

    println!("{}", square_sum - sum_squares);
}
