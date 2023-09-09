// https://adventofcode.com/2015/day/2

// For example:

// A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
// All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?


fn calc_box_siz(firstnumber: number, secondnumber: number, thirdnumber: number) {
    let firstside = firstnumber * secondnumber;
    let secondside = secondnumber * thirdnumber;
    let thirdside = thirdnumber * firstnumber;
    let first_dimension = firstside * 2;
    let second_dimension = secondside * 2;
    let third_dimension = thirdside * 2; 
    return first_dimension + second_dimension + third_dimension;
}


fn main() {
}
