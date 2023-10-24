// https://adventofcode.com/2015/day/3

// Santa is delivering presents to an infinite two-dimensional grid of houses.

// He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

// However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

// For example:

// > delivers presents to 2 houses: one at the starting location, and one to the east.
// ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
// ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

fn main() {
let input: String = std::fs::read_to_string("src/advent-of-code15/day-three-input.txt").expect("couldn't read the file, check if day-three-input.txt exists.");
let mut deliveried_houses: i32 = 1;
let north: char = '^';
let south: char = 'v';
let east: char = '>';
let west: char = '<';
let mut current_position: (i32, i32) = (0, 0);
let mut visited_houses: Vec<(i32, i32)> = vec![(0, 0)];
for c in input.chars() {
    if c == north {
        current_position.1 += 1;
    } else if c == south {
        current_position.1 -= 1;
    } else if c == east {
        current_position.0 += 1;
    } else if c == west {
        current_position.0 -= 1;
    }
    if !visited_houses.contains(&current_position) {
        deliveried_houses += 1;
        visited_houses.push(current_position);
    }
}
println!("{}", deliveried_houses);
}