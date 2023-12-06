use std::fs::read_to_string;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = args.get(1).expect("No input provided");
    let lines = read_to_string(input).expect("failed to read input file");

    let mut sum = 0;

    for line in lines.lines() {
        let digits = to_digits_2(line);
        let left = digits.get(0).expect("no digit at index 0?");
        let right = digits.get(digits.len() - 1).expect("no digit at index -1?");

        sum += left * 10 + right;
    }
    println!("sum: {}", sum);
}

fn to_digits(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for char in line.chars() {
        if char.is_digit(10) {
            digits.push(char.to_digit(10).expect("failed to parse digit"));
        }
    }
    digits
}

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_digits_2(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            digits.push(char.to_digit(10).expect("failed to parse digit"));
        } else {
            for (w_index, &word) in NUMBERS.iter().enumerate() {
                if line[index..].starts_with(word) {
                    digits.push((w_index + 1) as u32);
                }
            }
        }
    }
    digits
}
