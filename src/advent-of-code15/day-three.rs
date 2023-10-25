// https://adventofcode.com/2015/day/3

// The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

// Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

// This year, how many houses receive at least one present?

// For example:

// ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
// ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
// ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

fn main() {
let input: String = std::fs::read_to_string("src/advent-of-code15/day-three-input.txt").expect("couldn't read the file, check if day-three-input.txt exists.");

let north: char = '^';
let south: char = 'v';
let east: char = '>';
let west: char = '<';

let mut deliveried_houses: i32 = 1;
let mut santa_position: (i32, i32) = (0, 0);
let mut robot_position: (i32, i32) = (0, 0);
let mut visited_houses: Vec<(i32, i32)> = vec![(0, 0)];
let mut index = 1;

for c in input.chars() {
    if index % 2 == 1 {
        if c == north {
            santa_position.1 += 1;
        } else if c == south {
            santa_position.1 -= 1;
        } else if c == east {
            santa_position.0 += 1;
        } else if c == west {
            santa_position.0 -= 1;
        }
        if !visited_houses.contains(&santa_position) {
            deliveried_houses += 1;
            visited_houses.push(santa_position);
        }
    } else {
        if c == north {
            robot_position.1 += 1;
        } else if c == south {
            robot_position.1 -= 1;
        } else if c == east {
            robot_position.0 += 1;
        } else if c == west {
            robot_position.0 -= 1;
        }
        if !visited_houses.contains(&robot_position) {
            deliveried_houses += 1;
            visited_houses.push(robot_position);
        }
    }
    index += 1;
}
println!("{}", deliveried_houses);
}