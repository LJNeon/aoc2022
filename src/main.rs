use prettytable::*;

mod day01;
mod day02;

fn main() {
  let mut table = Table::new();

  table.set_titles(row![cFc => "Day", "Answer A", "Answer B"]);
  table.add_row(row!["Day 1", day01::solve_a(), day01::solve_b()]);
  table.add_row(row!["Day 2", day02::solve_a(), day02::solve_b()]);

  table.set_format(format::FormatBuilder::new()
    .column_separator('│')
    .borders('│')
    .separators(&[format::LinePosition::Top], format::LineSeparator::new('─', '┬', '┌', '┐'))
    .separators(&[format::LinePosition::Intern], format::LineSeparator::new('─', '┼', '├', '┤'))
    .separators(&[format::LinePosition::Bottom], format::LineSeparator::new('─', '┴', '└', '┘'))
    .padding(1, 1)
    .build());
  table.printstd();
}
