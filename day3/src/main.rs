struct SchematicLine {
    number_position: Vec<bool>,
    symbol_position: Vec<bool>,
}

impl SchematicLine {
    fn new(line: &str) -> Self {
        let mut number_position: Vec<bool> = vec![false; line.len()];
        let mut symbol_position: Vec<bool> = vec![false; line.len()];
        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                number_position[index] = true;
            } else if char != '.' {
                symbol_position[index] = true;
            }
        }
        SchematicLine {
            number_position,
            symbol_position,
        }
    }
}
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
fn main() {
    let input = include_str!("input.txt");
    let mut lines = input
        .lines()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut matrix = lines.iter().enumerate();
    let top_row = matrix.next().unwrap();
    for (row_index, row) in matrix {
        println!("{row_index} {row:?}");
        let symbol_positions = row
            .iter()
            .enumerate()
            .filter_map(|(index, e)| {
                if !e.is_numeric() && *e != '.' {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        println!("{symbol_positions:?}");
        for symbol_pos in symbol_positions.iter() {
            if top_row.1[*symbol_pos] == '.' {
                if top_row.1[*symbol_pos - 1].is_numeric() {
                    let num: String = top_row.1[..*symbol_pos].iter().rev().take_while(|num| num.is_numeric()).collect::<String>();
                    // TODO deal with this unwrap
                    let num = num.chars().rev().collect::<String>().parse::<usize>().unwrap();
                    println!("{num:?}");
                }
                if top_row.1[*symbol_pos + 1].is_numeric() {
                    let num = top_row.1[*symbol_pos + 1..].iter().take_while(|num| num.is_numeric()).collect::<String>().parse::<usize>().unwrap();
                    // TODO deal with this unwrap
                    //let num = num.chars().rev().collect::<String>().parse::<usize>().unwrap();
                    println!("{num:?}");
                }
            }
            // if top_row.1[*symbol_pos - 1].is_numeric() && top_row.1[*symbol_pos] {

            // }
        }
        // if i.iter().any(|e| !e.is_numeric() && *e != '.') {
        //     println!("{i:?}");
        // }
    }
    // let mut top_line = SchematicLine::new(lines.next().unwrap());
    // for line in lines {
    //     let current_line = SchematicLine::new(line);
    //     let asd = current_line.number_position.iter().enumerate().filter(|(i, e)| **e == true).map(|(i, _)| i).collect::<Vec<_>>();
    //     println!("{asd:?}");
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schematic_line_new() {
        let schematic_line = SchematicLine::new("-....+.58.");
        assert_eq!(
            schematic_line.number_position,
            vec![false, false, false, false, false, false, false, true, true, false]
        );
        assert_eq!(
            schematic_line.symbol_position,
            vec![true, false, false, false, false, true, false, false, false, false]
        );
    }
}
