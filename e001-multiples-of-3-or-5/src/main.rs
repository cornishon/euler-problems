// Sum of all multiples of 3 or 5 below 1000

fn main() {
    let mut sum: i32 = 0;

    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    println!("The sum of all multiples of 3 or 5 below 1000 equals {}.", sum);
    
}
