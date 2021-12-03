const INPUT: &str = include_str!("../../inputs/day2.txt");

fn calculate_position(input: &str) -> (u32, u32) {
    let mut position: u32 = 0;
    let mut depth: u32 = 0;

    for line in input.trim().lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let value: u32 = values[1].trim().parse().unwrap();

        match values[0].trim() {
            "forward" => position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unexpected match"),
        }
    }

    (position, depth)
}

fn main() {
    let (pos, depth) = calculate_position(INPUT);

    println!("Position times depth is {}", pos * depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_calculate_correct_position() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let (pos, depth) = calculate_position(input);

        assert_eq!(pos * depth, 150)
    }
}
