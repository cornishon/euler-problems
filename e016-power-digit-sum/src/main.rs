fn main() {
  
  #[allow(arithmetic_overflow)]
  fn shift() {
    println!("{}", 2_i128.pow(1000));
  }
  
  shift();
  
}
