use std::collections::HashMap;
use std::io::stdin;

fn readln() -> String {
  let mut s = String::new();
  stdin().read_line(&mut s).unwrap();
  s
}

// https://yukicoder.me/problems/no/227
pub fn run() {
  let hand = readln()
    .trim()
    .split_whitespace()
    .map(|card| card.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  let count_map = hand
    .into_iter()
    .fold(HashMap::new(), |mut hm: HashMap<i32, i32>, card| {
      {
        let entry = hm.entry(card).or_insert(0);
        *entry += 1;
      }
      hm
    });
  let count_of_count_map = count_map
    .values()
    .fold(HashMap::new(), |mut hm: HashMap<i32, i32>, count| {
      {
        let entry = hm.entry(*count).or_insert(0);
        *entry += 1;
      }
      hm
    });

  match (count_of_count_map.get(&3), count_of_count_map.get(&2)) {
    (Some(&1), Some(&1)) => println!("FULL HOUSE"),
    (Some(&1), None    ) => println!("THREE CARD"),
    (None    , Some(&2)) => println!("TWO PAIR"),
    (None    , Some(&1)) => println!("ONE PAIR"),
    _                    => println!("NO HAND"),
  };
}
