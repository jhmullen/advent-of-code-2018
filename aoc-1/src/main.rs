use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
  let f = File::open("input.txt").expect("io error");
  let reader = BufReader::new(f);

  let mut freq = 0;

  for line in reader.lines() {  
    let ln:String = line.unwrap();
    let diff:i32 = ln.parse::<i32>().unwrap();
    freq += diff;
  }

  println!("{}", freq);

}