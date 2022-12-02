pub fn solve_a() -> u16 {
  include_str!("../inputs/day02.txt")
    .lines()
    .map(|game| match game {
      "A X" => 4,
      "B X" => 1,
      "C X" => 7,
      "A Y" => 8,
      "B Y" => 5,
      "C Y" => 2,
      "A Z" => 3,
      "B Z" => 9,
      "C Z" => 6,
      _ => unreachable!()
    })
    .sum()
}

pub fn solve_b() -> u16 {
  include_str!("../inputs/day02.txt")
    .lines()
    .map(|game| match game {
      "A X" => 3,
      "B X" => 1,
      "C X" => 2,
      "A Y" => 4,
      "B Y" => 5,
      "C Y" => 6,
      "A Z" => 8,
      "B Z" => 9,
      "C Z" => 7,
      _ => unreachable!()
    })
    .sum()
}
