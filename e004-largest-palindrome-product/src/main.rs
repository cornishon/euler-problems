fn is_palindrome(n: u64) -> bool {
    let n1: String = n.to_string().chars().rev().collect();
    let n2: String = n.to_string().chars().collect();
    n1 == n2
}

fn main() {
    'l: for i in (900..=999).rev() {
        for j in (900..=999).rev() {
            if is_palindrome(i*j) {
                println!("{}", i*j);
                break 'l;
            }
        }
    }
}
