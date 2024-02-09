use std::num::ParseIntError;

struct MinSetOfCubes {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    game_number: usize,
    is_game_possible: Option<bool>
}

impl MinSetOfCubes {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        MinSetOfCubes { red, green, blue }
    }

    fn update_min_size(&mut self, num: usize, color: &str) {
        match color {
            "red" => {
                if num > self.red {
                    self.red = num
                }
            }
            "green" => {
                if num > self.green {
                    self.green = num
                }
            }
            "blue" => {
                if num > self.blue {
                    self.blue = num
                }
            }
            _ => panic!("unexpected color: {color}"),
        }
    }
}

impl Game {
    const RED: usize = 12;
    const GREEN: usize = 13;
    const BLUE: usize = 14;

    fn new(game_number: usize, is_game_possible: Option<bool>) -> Self {
        Game { game_number, is_game_possible }
    }

    fn set_games_possibility(&mut self, num: usize, color: &str) {
        if self.is_game_possible == Some(false) {
            return;
        }
        self.is_game_possible = match color {
            "red" => Some(num <= Self::RED),
            "green" => Some(num <= Self::GREEN),
            "blue" => Some(num <= Self::BLUE),
            _ => panic!("unexpected color: {color}"),
        }
    }
}

fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");
    let mut part1_result: usize = 0;
    let mut part2_result: usize = 0;
    for line in input.lines() {
        let mut split_line = line.split(':');
        let (game_number, sets) = (split_line.next().unwrap(), split_line.next().unwrap());
        let game_number =
            extract_game_number(game_number).map_err(|e| eprintln!("unable to parse: {e}"))?;
        let mut min_set_of_cubes = MinSetOfCubes::new(0, 0, 0);
        let mut game = Game::new(game_number, None);
        for set in sets.split(';') {
            for subset in set.split(',') {
                let mut subset = subset.split_whitespace();
                let (num, color) = (
                    subset.next().unwrap().parse::<usize>().unwrap(),
                    subset.next().unwrap(),
                );
                game.set_games_possibility(num, color);
                min_set_of_cubes.update_min_size(num, color);
            }
        }
        if let Some(true) = game.is_game_possible {
            part1_result += game.game_number;
        };
        part2_result += min_set_of_cubes.red * min_set_of_cubes.green * min_set_of_cubes.blue;
    }
    println!("{part1_result}");
    println!("{part2_result}");
    Ok(())
}

fn extract_game_number(line: &str) -> Result<usize, ParseIntError> {
    let sp_byte = line.find(' ').unwrap();
    line[sp_byte + 1..].parse::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_game_number_single_digit() {
        assert_eq!(extract_game_number("Game 5").unwrap(), 5)
    }
    #[test]
    fn test_extract_game_number_double_digit() {
        assert_eq!(extract_game_number("Game 55").unwrap(), 55)
    }

    #[test]
    fn test_is_game_possbile() {
        let mut game = Game { game_number: 1, is_game_possible: None};
        game.set_games_possibility(2, "red");
        assert_eq!(game.is_game_possible.unwrap(), true);
        game.set_games_possibility(15, "red");
        game.set_games_possibility(2, "red");
        game.set_games_possibility(16, "blue");
        assert_eq!(game.is_game_possible.unwrap(), false);
    }

    #[test]
    fn test_update_cubes_min_set_size() {
        let mut cube_set = MinSetOfCubes { red: 0, green: 0, blue: 0};
        cube_set.update_min_size(2, "red");
        cube_set.update_min_size(3, "blue");
        assert_eq!(cube_set.red, 2);
        assert_eq!(cube_set.blue, 3);
        assert_eq!(cube_set.green, 0);
    }
}
