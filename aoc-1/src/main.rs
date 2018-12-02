use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn main() {
  part1();
  part2();
}

fn part1() {
  let f = File::open("input.txt").expect("io error");
  let reader = BufReader::new(f);

  let mut freq = 0;

  for line in reader.lines() {  
    let ln:String = line.unwrap();
    let diff:i32 = ln.parse::<i32>().unwrap();
    freq += diff;
  }

  println!("part 1: {}", freq);

} 

fn part2() {
  let mut f = File::open("input.txt").expect("io error");

  let mut freq = 0;
  let mut contents = String::new();
  let mut done = false;
  let mut totals = HashSet::new();

  f.read_to_string(&mut contents).expect("reading error");

  totals.insert(0);

  while !done {
    for line in contents.lines() {
      let diff:i32 = line.parse::<i32>().unwrap();
      freq += diff;
      if totals.contains(&freq) {
        done = true;
        println!("part 2: {}", freq);
        break;
      }
      else {
        totals.insert(freq);
      }
    }
  }
} 