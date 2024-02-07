use std::num::ParseIntError;


static BAG_OF_CUBES: BagOfCubes = BagOfCubes { red: 12, green: 13, blue: 14 };

struct BagOfCubes {
    red: usize,
    green: usize,
    blue: usize
}

impl BagOfCubes {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        BagOfCubes { red, green, blue }
    }
}


fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");
    let mut part1_result: usize = 0;
    let mut part2_result: usize = 0;
    for line in input.lines() {
        println!("{line}");
        let mut split_line = line.split(":");
        let (game_number, sets) = (split_line.next().unwrap(), split_line.next().unwrap());
        let game_number = extract_game_number(game_number).map_err(|e| eprintln!("unable to parse: {e}"))?;
        if is_game_possible(sets) {
            part1_result += game_number;
        }
       // part2_result += get_min_cubes_power_set()
    }
    println!("{part1_result}");
    Ok(())
}


fn extract_game_number(line: &str) -> Result<usize, ParseIntError> {
    let sp_byte = line.find(" ").unwrap();
    line[sp_byte + 1..].parse::<usize>()
}

 // part 1
fn is_game_possible(sets: &str) -> bool {
    for set in sets.split(";") {
        println!("{set}");
        for subset in set.split(",") {
            let mut subset = subset.split_whitespace();
            let (num, color) = (subset.next().unwrap().parse::<usize>().unwrap(), subset.next().unwrap());
            match color {
                "red" => if num > 12 { return false } else { continue; }
                "green" => if num > 13 { return false } else { continue; }
                "blue" => if num > 14 { return false } else { continue; }
                _ => panic!("unexpected color: {color}")
            }
        }
    } 
    true
}

// part 2
fn get_min_cubes_power_set(sets: &str) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_game_number_single_digit() {
        assert_eq!(extract_game_number("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(), 5)
    }
    #[test]
    fn test_extract_game_number_double_digit() {
        assert_eq!(extract_game_number("Game 55: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(), 55)
    }

    #[test]
    fn test_is_game_possbile() {
        assert_eq!(is_game_possible("Game 55: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), true);
        assert_eq!(is_game_possible("Game 55: 15 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), false)
    }
}