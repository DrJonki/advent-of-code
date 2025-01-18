use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("./2015/03/input.txt");
  let file = fs::read_to_string(path).unwrap();

  // Part 1
  {
    let mut map = HashSet::<(i32, i32)>::new();
    map.insert((0, 0));

    let mut current_coords: (i32, i32) = (0, 0);

    file.chars().for_each(|c| {
      match c {
        '>' => current_coords.0 += 1,
        '<' => current_coords.0 -= 1,
        '^' => current_coords.1 += 1,
        'v' => current_coords.1 -= 1,
        _ => return,
      };

      map.insert(current_coords);
    });

    println!("P1: {}", map.len());
  }

  // Part 2
  {
    let mut map = HashSet::<(i32, i32)>::new();
    map.insert((0, 0));

    let mut current_coords_santa: (i32, i32) = (0, 0);
    let mut current_coords_robo: (i32, i32) = (0, 0);

    for (i, c) in file.chars().enumerate() {
      let current_coords = if i % 2 == 0 {
        &mut current_coords_santa
      } else {
        &mut current_coords_robo
      };

      match c {
        '>' => current_coords.0 += 1,
        '<' => current_coords.0 -= 1,
        '^' => current_coords.1 += 1,
        'v' => current_coords.1 -= 1,
        _ => return,
      };

      map.insert(*current_coords);
    }

    println!("P2: {}", map.len());
  }
}
