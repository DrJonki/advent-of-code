use regex::Regex;

fn main() {
  let input = include_str!("input.txt");

  // Part 1
  {
    let matcher = Regex::new(r#"\\x[0-9a-f]{2}|\\"|\\\\|[a-z]"#).unwrap();
    let mut literals = 0;
    let mut bytes = 0;

    for line in input.lines() {
      literals += line.len();
      bytes += matcher.captures_iter(line).count();
    }

    println!("P1: {}", literals - bytes);
  }

  // Part 2
  {
    let matcher = Regex::new(r#"\\x[0-9a-f]{2}|"|\\"#).unwrap();
    let mut input_literals = 0;
    let mut output_literals = 0;

    for line in input.lines() {
      let line_len = line.len();

      input_literals += line_len;
      output_literals += line_len + matcher.captures_iter(line).count() + 2;
    }

    println!("P2: {}", output_literals - input_literals);
  }
}
