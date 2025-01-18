use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("./2015/01/input.txt");
  let file = fs::read_to_string(path).unwrap();

  // Part 1
  {
    let floor = file.chars().fold(0, |acc, c| match c {
      '(' => acc + 1,
      ')' => acc - 1,
      _ => acc,
    });

    println!("P1: {}", floor);
  }

  // Part 2
  {
    let mut floor = 0;

    for (i, c) in file.chars().enumerate() {
      match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => {}
      };

      if floor < 0 {
        println!("P2: {}", i + 1);
        break;
      }
    }
  }
}
