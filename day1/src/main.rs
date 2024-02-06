static WORDS: [&str; 9] = [
    "nine",
    "eight",
    "seven",
    "six",
    "five",
    "four",
    "three",
    "two",
    "one"
];

pub fn main() {
    let input = include_str!("input.txt");
    let sum: u32 = calculate_calibration(input);
    println!("{sum}");
}

fn calculate_calibration(input: &str) -> u32 {
    let mut results = Vec::new();
    for line in input.lines() {
        let digits = get_all_digits(line);
        let res: u32 = match digits.len() {
            1 => format!("{}{}", digits.first().unwrap(), digits.first().unwrap()).parse().unwrap(),
            _ => format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap()
        };
        results.push(res);
    }
    results.iter().sum()
}

fn get_all_digits(txt: &str) -> Vec<char> {
    let mut new_line = txt.to_string();

    for word in WORDS {
        while let Some(starting_byte) = new_line.find(word) {
            let matched_word = &new_line[starting_byte..starting_byte + word.len()];
            let digit = match matched_word {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("not a number: {matched_word}"),
            };
            new_line.replace_range(
                starting_byte + 1..starting_byte + (matched_word.len() - 1),
                digit,
            );
        }
    }
    new_line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_digits() {
        assert_eq!(
            get_all_digits("xtwone3four").iter().collect::<String>(),
            "2134"
        );
        assert_eq!(get_all_digits("two1nine").iter().collect::<String>(), "219");
        assert_eq!(
            get_all_digits("eightwothree").iter().collect::<String>(),
            "823"
        );
        assert_eq!(
            get_all_digits("abcone2threexyz").iter().collect::<String>(),
            "123"
        );
        assert_eq!(
            get_all_digits("4nineeightseven2")
                .iter()
                .collect::<String>(),
            "49872"
        );
        assert_eq!(
            get_all_digits("twofourthree778nineeight")
                .iter()
                .collect::<String>(),
            "24377898"
        );
        assert_eq!(
            get_all_digits("5qjlqp")
                .iter()
                .collect::<String>(),
            "5"
        );
        assert_eq!(
            get_all_digits("sixfconesix6three1sixsix")
                .iter()
                .collect::<String>(),
            "61663166"
        );
        
    }

    #[test]
    fn test_calculate_calibration() {
        assert_eq!(calculate_calibration("5qjlqp"), 55);
        assert_eq!(calculate_calibration("eightwothree"), 83);

    }
}
