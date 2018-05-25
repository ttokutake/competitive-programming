use std::io::stdin;

fn read_i32() -> i32 {
  let mut s = String::new();
  stdin().read_line(&mut s).unwrap();
  s.trim().parse::<i32>().unwrap()
}

// https://yukicoder.me/problems/no/661
pub fn run() {
  let count = read_i32();
  let mut inputs: Vec<i32> = vec![];
  for _ in 0..count {
    let input = read_i32();
    inputs.push(input);
  }
  for input in inputs {
    if input % 8 == 0 && input % 10 == 0 {
      println!("ikisugi");
    } else if input % 8 == 0 {
      println!("iki");
    } else if input % 10 == 0 {
      println!("sugi");
    } else {
      println!("{}", input / 3);
    }
  }
}
