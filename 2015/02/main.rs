use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
  let path = Path::new("./2015/02/input.txt");

  // Part 1
  {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let total_area = reader.lines().fold(0, |acc, line| {
      if let Ok(line) = line {
        let split: Vec<&str> = line.split("x").collect();
        let l = split[0].parse::<i32>().unwrap();
        let w = split[1].parse::<i32>().unwrap();
        let h = split[2].parse::<i32>().unwrap();

        let a = l * w;
        let b = w * h;
        let c = h * l;

        let min = a.min(b).min(c);

        return acc + 2 * a + 2 * b + 2 * c + min;
      }

      acc
    });

    println!("P1: {}", total_area);
  }

  // Part 2
  {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let total_length = reader.lines().fold(0, |acc, line| {
      if let Ok(line) = line {
        let mut split = line
          .split("x")
          .map(|dim| dim.parse::<i32>().unwrap())
          .collect::<Vec<i32>>();
        split.sort();

        let smallest = split[0];
        let second_smallest = split[1];
        let biggest = split[2];

        let ribbon = smallest * 2 + second_smallest * 2 + smallest * second_smallest * biggest;

        return acc + ribbon;
      }

      acc
    });

    println!("P2: {}", total_length);
  }
}
