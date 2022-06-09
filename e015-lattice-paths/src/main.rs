use std::io;

fn main() -> Result<(), io::Error> {
  println!("Input the size of the grid:");
  let mut buf = String::new();
  io::stdin().read_line(&mut buf)?;
  let n: u32 = buf.trim().parse()
    .expect("Not a positive integer!");
    
  println!("Number of paths: {}",
    paths(n));
    
  Ok(())
}

fn paths(n: u32) -> u32 {
  match n {
    1 => 2,
    _ => 2 * (paths(n-1) * 2_u32.pow(n-1)),
  }
}