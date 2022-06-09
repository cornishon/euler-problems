use std::fs;

fn main() {
  let filename = "large_numbers.txt";

  let f = fs::read_to_string(filename)
    .expect("Error reading file");
  
  let mut num = Vec::<Vec<u32>>::new();
  for line in f.lines() {
    let mut padded: Vec<u32> = vec![0, 0];
    for c in line.trim().chars() {
      let d = c.to_digit(10).unwrap();
      padded.push(d);
    }
    num.push(padded);
  }
  
  let mut sums = Vec::<Vec<u32>>::new();
  sums.push(num[0].clone());
  for n in &num[1..] {
    sums.push(add(&n, &sums.last().unwrap()));
  }
  
  println!("First 10 digits:");
  for d in sums.last().unwrap()
            .iter().take(10) {
    print!("{}", d);
  }
  println!("");
}


fn vec_rev(v: &Vec<u32>) -> Vec<u32> {
  v.iter().rev().cloned().collect()
}

fn add(v1: &Vec<u32>, v2: &Vec<u32>)
    -> Vec<u32> {
  let mut result: Vec<u32> = vec![];
  let mut carry: u32 = 0;
  for (d1, d2) in v1.iter().zip(v2).rev() {
    let mut sum = d1 + d2 + carry;
    carry = sum / 10;
    sum = sum % 10;
    result.push(sum);
  }
  result.push(carry);
  while result.len() > v2.len() {
    result.pop();
  }
vec_rev(&result)
}
