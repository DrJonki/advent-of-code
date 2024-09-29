use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("./2015/06/input.txt");

    let matcher =
        Regex::new(r"(?<action>toggle|(?:turn (?:off|on))) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)").unwrap();

    const COLUMNS: usize = 1000;
    const ROWS: usize = 1000;

    // Part 1
    {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut grid = vec![false; COLUMNS * ROWS];

        for line in reader.lines().map_while(Result::ok) {
            let caps = matcher.captures(&line).unwrap();

            type ActionFn = fn(&mut Vec<bool>, usize, usize) -> ();

            let action: Option<ActionFn> = match &caps["action"] {
                "toggle" => Some(|grid: &mut Vec<bool>, x: usize, y: usize| {
                    grid[y * COLUMNS + x] = !grid[y * COLUMNS + x];
                }),
                "turn on" => Some(|grid: &mut Vec<bool>, x: usize, y: usize| {
                    grid[y * COLUMNS + x] = true;
                }),
                "turn off" => Some(|grid: &mut Vec<bool>, x: usize, y: usize| {
                    grid[y * COLUMNS + x] = false;
                }),
                _ => None,
            };

            let x1 = caps["x1"].parse::<usize>().unwrap();
            let y1 = caps["y1"].parse::<usize>().unwrap();
            let x2 = caps["x2"].parse::<usize>().unwrap();
            let y2 = caps["y2"].parse::<usize>().unwrap();

            if let Some(action) = action {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        action(&mut grid, x, y);
                    }
                }
            }
        }

        let lights_on = grid.iter().fold(0, |acc, light| acc + *light as i32);

        println!("P1: {}", lights_on);
    }

    // Part 2
    {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut grid = vec![0u8; COLUMNS * ROWS];

        for line in reader.lines().map_while(Result::ok) {
            let caps = matcher.captures(&line).unwrap();

            type ActionFn = fn(&mut Vec<u8>, usize, usize) -> ();

            let action: Option<ActionFn> = match &caps["action"] {
                "toggle" => Some(|grid: &mut Vec<u8>, x: usize, y: usize| {
                    grid[y * COLUMNS + x] += 2;
                }),
                "turn on" => Some(|grid: &mut Vec<u8>, x: usize, y: usize| {
                    grid[y * COLUMNS + x] += 1;
                }),
                "turn off" => Some(|grid: &mut Vec<u8>, x: usize, y: usize| {
                    if grid[y * COLUMNS + x] > 0 {
                        grid[y * COLUMNS + x] -= 1;
                    }
                }),
                _ => None,
            };

            let x1 = caps["x1"].parse::<usize>().unwrap();
            let y1 = caps["y1"].parse::<usize>().unwrap();
            let x2 = caps["x2"].parse::<usize>().unwrap();
            let y2 = caps["y2"].parse::<usize>().unwrap();

            if let Some(action) = action {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        action(&mut grid, x, y);
                    }
                }
            }
        }

        let brightness = grid.iter().fold(0, |acc, light| acc + *light as i32);

        println!("P2: {}", brightness);
    }
}
