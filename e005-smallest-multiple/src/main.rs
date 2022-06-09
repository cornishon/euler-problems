fn main() {
    // Smallest number divisible by 1..20 is a number whose prime factorization
    // consists of all possible powers of primes that are below 20, that is
    // 19 * 17 * 13 * 11 * 7 * 5 * 3^2 * 2^4
    println!("{}", 19 * 17 * 13 * 11 * 7 * 5 * 3*3 * 2*2*2*2);
}
