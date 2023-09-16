// https://adventofcode.com/2015/day/2

// For example:

// 2*l*w + 2*w*h + 2*h*l
// A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
// All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
use std::fs::File;
use std::io::{BufRead, BufReader};

// fn calc_box_siz(content: Vec<&str>) -> i32 {
//     let box_length = content[0].parse::<i32>().unwrap();
//     let box_width = content[1].parse::<i32>().unwrap();
//     let box_height = content[2].parse::<i32>().unwrap();
//     let first_area = box_length * box_width;
//     let second_area = box_width * box_height;
//     let third_area = box_height * box_length;
//     let first_dimension = first_area * 2;
//     let second_dimension = second_area * 2;
//     let third_dimension = third_area * 2;
//     return first_dimension + second_dimension + third_dimension + [first_area, second_area, third_area].iter().min().unwrap();
// }

// A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
// A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

fn calc_ribbon_siz(content: Vec<i32>) -> i32 {
    let ribbon_height: i32 = content[0];
    let ribbon_width: i32 = content[1];
    let ribbon_length: i32 = content[2];
    let ribbon_wrapper: i32 = ribbon_height + ribbon_height + ribbon_width + ribbon_width;
    let ribbon_bow: i32 = ribbon_height * ribbon_width * ribbon_length;
    return ribbon_wrapper + ribbon_bow;
}


fn main() {
    let file:File = File::open("src/advent-of-code15/day-two-input.txt").expect("could not read file");
    let content: BufReader<File> = BufReader::new(file);
    let mut total: i32 = 0;
    for line in content.lines() {
        match line {
            Ok(line) => {
                let mut content: Vec<i32> = line
                .split("x")
                .into_iter()
                .map(|s| s.parse::<i32>().expect("couldn't parse the string"))
                .collect();
                content.sort();
                total += calc_ribbon_siz(content)
            }
            _ => ()
        }
    }
    println!("{}", total)
}
