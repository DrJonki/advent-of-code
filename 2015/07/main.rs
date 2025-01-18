use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Operand<'a> {
  Wire(&'a str),
  Const(u16),
}

#[derive(Debug)]
enum Op<'a> {
  Signal(u16),
  Mov(&'a str),
  And(Operand<'a>, &'a str),
  Or(&'a str, &'a str),
  LShift(&'a str, u16),
  RShift(&'a str, u16),
  Not(&'a str),
}

fn main() {
  let input = include_str!("input.txt");

  let lines = input.lines();
  let matcher = Regex::new(r"(?<not>NOT)? ?(?<src1>[\w\d]+) ?(?<op>AND|OR|LSHIFT|RSHIFT)? ?(?<src2>[\w\d]+)? -> (?<dest>\w+)").unwrap();

  println!("Lines: {}", lines.clone().count());

  type OpMap<'a> = HashMap<&'a str, Op<'a>>;
  let mut op_map: OpMap = HashMap::new();

  for line in lines.into_iter() {
    let caps = matcher
      .captures(line)
      .unwrap_or_else(|| panic!("Invalid line: {}", line));

    let not = caps.name("not");
    let src1 = caps.name("src1").unwrap().as_str();
    let op = caps.name("op");
    let src2 = caps.name("src2");
    let dest = caps.name("dest").unwrap().as_str();

    op_map.insert(
      dest,
      if not.is_some() {
        Op::Not(src1)
      } else if op.is_some() {
        let op = op.unwrap().as_str();
        let src2 = src2
          .unwrap_or_else(|| panic!("Right operand not found for op {}", op))
          .as_str();

        match op {
          "AND" => Op::And(
            match src1.parse::<u16>() {
              Ok(value) => Operand::Const(value),
              Err(_) => Operand::Wire(src1),
            },
            src2,
          ),
          "OR" => Op::Or(src1, src2),
          "LSHIFT" => Op::LShift(
            src1,
            src2
              .parse::<u16>()
              .expect("LShift expects right operand as number"),
          ),
          "RSHIFT" => Op::RShift(
            src1,
            src2
              .parse::<u16>()
              .expect("LShift expects right operand as number"),
          ),
          _ => panic!("Unknown operation {}", op),
        }
      } else {
        let signal = src1.parse::<u16>();

        match signal {
          Ok(signal) => Op::Signal(signal),
          Err(_) => Op::Mov(src1),
        }
      },
    );
  }

  fn process_op<'a>(
    op_map: &'a OpMap,
    value_map: &mut HashMap<&'a str, u16>,
    wire: &'a str,
  ) -> u16 {
    if let Some(value) = value_map.get(wire) {
      return *value;
    }

    let op = op_map
      .get(wire)
      .unwrap_or_else(|| panic!("Wire '{}' not found", wire));

    let value = match op {
      Op::Signal(signal) => *signal,
      Op::Mov(wire) => process_op(op_map, value_map, wire),
      Op::And(left, right) => {
        (match left {
          Operand::Const(value) => *value,
          Operand::Wire(wire) => process_op(op_map, value_map, wire),
        }) & process_op(op_map, value_map, right)
      }
      Op::Or(left, right) => {
        process_op(op_map, value_map, left) | process_op(op_map, value_map, right)
      }
      Op::LShift(left, bits) => process_op(op_map, value_map, left) << bits,
      Op::RShift(left, bits) => process_op(op_map, value_map, left) >> bits,
      Op::Not(wire) => !process_op(op_map, value_map, wire),
    };

    value_map.insert(wire, value);

    value
  }

  // Part 1
  let value_a = {
    let mut value_map = HashMap::new();

    let value_a = process_op(&op_map, &mut value_map, "a");
    println!("P1: {}", value_a);

    value_a
  };

  // Part 2
  {
    let mut value_map = HashMap::new();

    op_map.insert("b", Op::Signal(value_a));

    let value_b = process_op(&op_map, &mut value_map, "a");
    println!("P2: {}", value_b);
  }
}
