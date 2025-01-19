use itertools::Itertools;
use regex::Regex;
use std::{
  collections::{HashMap, HashSet},
  usize,
};

fn main() {
  let input = include_str!("input.txt");
  let matcher = Regex::new(r#"(\w+) to (\w+) = (\d+)"#).unwrap();

  let mut cities = HashSet::<&str>::new();
  let mut distances = HashMap::<(&str, &str), u8>::new();

  // Part 1 & 2
  {
    for line in input.lines() {
      if let Some(caps) = matcher.captures(line) {
        let (_, [from, to, dist]) = caps.extract();
        let from_to = [from, to];
        let sorted: Vec<_> = from_to.iter().sorted().collect();
        let from = sorted.get(0).unwrap();
        let to = sorted.get(1).unwrap();

        cities.insert(from);
        cities.insert(to);
        distances.insert((from, to), dist.parse().unwrap());
      }
    }

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for perm in cities.iter().permutations(cities.len()) {
      let dist = perm[..].windows(2).fold(0usize, |acc, from_to| {
        let sorted: Vec<_> = from_to.iter().sorted().collect();
        let from = sorted.get(0).unwrap();
        let to = sorted.get(1).unwrap();
        let dist = *distances.get(&(from, to)).unwrap() as usize;

        acc + dist
      });

      min = min.min(dist);
      max = max.max(dist);
    }

    println!("P1: {}", min);
    println!("P2: {}", max);
  }
}
