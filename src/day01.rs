pub fn solve_a() -> u32 {
  include_str!("../inputs/day01.txt")
    .trim()
    .split("\n\n")
    .map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
    .max()
    .unwrap()
}

pub fn solve_b() -> u32 {
  let mut elves: Vec<u32> = include_str!("../inputs/day01.txt")
    .trim()
    .split("\n\n")
    .map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
    .collect();

  elves.sort_unstable();

  elves.iter()
    .rev()
    .take(3)
    .sum()
}
