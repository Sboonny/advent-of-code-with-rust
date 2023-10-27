// https://adventofcode.com/2015/day/4

// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

// For example:

// If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
// If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
use md5::{Md5, Digest};

fn main() {
    let decoded_string: &str = "bgvyzdsv";
    let pattern =  "00000";
    let mut i = 0;
    let mut found = false;
    while !found {
        let mut hasher = Md5::new();
        hasher.update(decoded_string.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize();
        let result_string = format!("{:x}", result);
        if result_string.starts_with(pattern) {
            found = true;
            println!("Found it! {}", i);
        }
        i += 1;
    }
}

