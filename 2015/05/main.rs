use regex::{Regex, RegexSet};
use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
};

fn main() {
  let path = Path::new("./2015/05/input.txt");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  let lines: Vec<_> = reader.lines().filter(|line| line.is_ok()).collect();

  // Part 1
  {
    let match_vowels = Regex::new(r"[aeiou]").unwrap();
    let match_consecutive = RegexSet::new([
      r"aa", r"bb", r"cc", r"dd", r"ee", r"ff", r"gg", r"hh", r"ii", r"jj", r"kk", r"ll", r"mm",
      r"nn", r"oo", r"pp", r"qq", r"rr", r"ss", r"tt", r"uu", r"vv", r"ww", r"xx", r"yy", r"zz",
    ])
    .unwrap();
    let match_forbidden = Regex::new(r"(ab|cd|pq|xy)+").unwrap();

    let nice = lines.iter().fold(0, |acc, line| {
      let line = line.as_deref().unwrap();

      acc
        + (match_vowels.find_iter(line).count() >= 3
          && match_consecutive.is_match(line)
          && !match_forbidden.is_match(line)) as i32
    });

    println!("P1: {}", nice);
  }

  // Part 2
  {
    let nice = lines.iter().fold(0, |acc, line| {
      let line = line.as_deref().unwrap();
      let bytes: Vec<_> = line.bytes().collect();

      let mut nice = false;

      'outer: for i in 0..=bytes.len() - 2 {
        for j in i + 2..=bytes.len() - 2 {
          if bytes[i..i + 2] == bytes[j..j + 2] {
            nice = true;
            break 'outer;
          }
        }
      }

      if nice {
        for i in 0..bytes.len() - 2 {
          if bytes[i] == bytes[i + 2] {
            return acc + 1;
          }
        }
      }

      acc
    });

    println!("P2: {}", nice);
  }
}
