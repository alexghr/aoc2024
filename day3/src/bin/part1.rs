use day3::Schematic;

fn main() {
    let input = include_str!("../../input/part1.txt");
    let result = process(input);
    println!("Result (day 3/part 1): {}", result);
}

fn process(input: &str) -> String {
    let schematic: Schematic = input.parse().unwrap();
    let sum = schematic.sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..\
"
            ),
            "4361"
        );
    }
}
