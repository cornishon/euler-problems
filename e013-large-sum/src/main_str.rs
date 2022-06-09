use std::fs;

fn main() {
  let filename = "large_numbers.txt";

  let f = fs::read_to_string(filename)
    .expect("Error reading file");
  
  let mut num = Vec::<String>::new();
  for line in f.lines() {
    let mut padded = String::from("00");
    padded.push_str(line.trim());
    num.push(padded);
    
  }
  
  let mut sums = Vec::<String>::new();
  sums.push(num[0].clone());
  for n in &num[1..] {
    sums.push(add(&n, &sums.last().unwrap()));
  }
  
  println!("First 10 digits:");
  for c in sums.last().unwrap()
            .chars().take(10) {
    print!("{}", c);
  }
  println!("");
}


fn rev_str(s: &String) -> String {
  s.chars().rev().collect::<String>()
}

fn add(v1: &String, v2: &String)
    -> String {
  let mut result = String::new();
  let mut carry: u32 = 0;
  for (c1, c2) in rev_str(v1).chars().zip(rev_str(v2).chars()) {
    let d1 = c1.to_digit(10).unwrap();
    let d2 = c2.to_digit(10).unwrap();
    let mut sum = d1 + d2 + carry;
    carry = sum / 10;
    sum = sum % 10;
    result.push_str(&format!("{}", sum));
  }
  let carry_s = rev_str(&format!("{}", carry));
  result.push_str(&carry_s);
  while result.len() > v2.len() {
    result.pop();
  }
  rev_str(&result)
}

