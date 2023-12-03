// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

use std::fs;

fn main() {
  let input = fs::read_to_string("src/advent-of-code23/day-one-input.txt").expect("");
  let mut sum: u32 = 0;
  for line in input.lines() {
      let digit = line.chars();
      let first_digit = digit.clone().filter(|char| char.is_numeric()).next().unwrap().to_digit(10).unwrap();
      let second_digit = digit.clone().filter(|char| char.is_numeric()).last().unwrap().to_digit(10).unwrap();
      sum += first_digit* 10 +  second_digit;
  }
  print!("{}", sum)
}
