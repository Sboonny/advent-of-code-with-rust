// https://adventofcode.com/2015/day/5
// Santa needs help figuring out which strings in his text file are naughty or nice.

// A nice string is one with all of the following properties:

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
// For example:

// ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
// aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
// jchzalrnumimnmhp is naughty because it has no double letter.
// haegwjzuvuyypxyu is naughty because it contains the string xy.
// dvszwmarrgswjxmb is naughty because it contains only one vowel.
// How many strings are nice?

fn main() {
    let input: String = std::fs::read_to_string("src/advent-of-code15/day-five-input.txt")
        .expect("couldn't read the file, check if day-three-input.txt exists.");

    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let forbidden_char: [&str; 4] = ["ab", "cd", "pq", "xy"];
    let mut the_amount_of_nice_strings: i32 = 0;

    for line in input.lines() {
        let mut vowel_count: i32 = 0;
        let mut double_letter: bool = false;
        let mut forbidden_string: bool = false;
        let mut last_char: char = ' ';
        for c in line.chars() {
            if vowels.contains(&c) {
               vowel_count += 1;
            }
            if c == last_char {
                double_letter = true;
            }
            if forbidden_char.contains(&format!("{}{}", last_char, c).as_str()) {
                forbidden_string = true;
            }
            last_char = c;
        }
        if vowel_count >= 3 && double_letter && !forbidden_string {
            the_amount_of_nice_strings += 1;
        }
    }
    print!("The amount of nice strings is: {}", the_amount_of_nice_strings)
}
