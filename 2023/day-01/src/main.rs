use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut result = 0;

    for line in fs::read_to_string("src/input").unwrap().lines() {
        let mut number: String = "".to_string();
        let mut first = '0';
        let mut last = '0';

        // turn line into iterator
        for character in line.chars() {
            if character.is_ascii_digit() {
                first = if first == '0' { character } else { first };
                last = character;
            }
        }
        number = number + &first.to_string() + &last.to_string();
        result += number.parse::<u32>().unwrap();
    }

    println!("part 1: {}", result)
}

fn part2() {
    let mut result = 0;

    for line in fs::read_to_string("src/input").unwrap().lines() {
        let mut number: String = "".to_string();
        let mut first = '0';
        let mut last = '0';

        // turn line into iterator
        for character in line.char_indices() {
            let n: String = "".to_string();
            if character.1.is_ascii_digit() {
                first = if first == '0' { character.1 } else { first };
                last = character.1;
            } else {
                first = if first == '0' {
                    let option_first =
                        find_number_word(line.split_at(character.0).1, number.clone());
                    if option_first.is_some() {
                        option_first.unwrap()
                    } else {
                        '0'
                    }
                } else {
                    first
                };
                let option_last = find_number_word(line.split_at(character.0).1, n);
                last = if option_last.is_some() {
                    option_last.unwrap()
                } else {
                    last
                };
            }
        }

        number = number + &first.to_string() + &last.to_string();
        result += number.parse::<u32>().unwrap();
    }

    println!("part 2: {}", result)
}

fn find_number_word(word: &str, number: String) -> Option<char> {
    if !word.is_empty() {
        let new_number: String = number + &word.chars().nth(0).unwrap().to_string();
        return match new_number.as_str() {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            _ => find_number_word(&word.split_at(1).1, new_number),
        };
    } else {
        None
    }
}
