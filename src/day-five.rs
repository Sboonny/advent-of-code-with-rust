// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

// Now, a nice string is one with all of the following properties:

// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
// For example:

// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
// xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
// uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
// ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
// How many strings are nice under these new rules?

fn main() {
    let input: String = std::fs::read_to_string("src/advent-of-code15/day-five-input.txt")
        .expect("couldn't read the file, check if day-three-input.txt exists.");

    let mut nice_strings = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut pair_found = false;
        let mut repeat_found = false;

        for i in 0..chars.len() - 1 {
            let pair = &chars[i..i + 2];

            if i < chars.len() - 2 && chars[i] == chars[i + 2] {
                repeat_found = true;
            }

            for j in i + 2..chars.len() - 1 {
                if &chars[j..j + 2] == pair {
                    pair_found = true;
                    break;
                }
            }

            if pair_found && repeat_found {
                break;
            }
        }

        if pair_found && repeat_found {
            nice_strings += 1;
        }
    }

    println!("Number of nice strings: {}", nice_strings);
}
