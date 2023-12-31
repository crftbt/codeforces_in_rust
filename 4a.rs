# 30ms 0KB mem

use std::io;

fn take_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}
  
pub fn main() {
  let w = take_int();
  if w == 2 {
    println!("NO");
  } else if w % 2 == 0 {
    println!("YES");
  } else {
    println!("NO");
  }
}
