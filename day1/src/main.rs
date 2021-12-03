use adventlib;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn get_value_increases(input: Vec<u32>) -> u32 {
    let mut increase_count: u32 = 0;
    let mut previous_value: u32 = u32::MAX;

    for value in input {
        if value > previous_value {
            increase_count += 1;
        }

        previous_value = value;
    }

    increase_count
}

fn get_sliding_window_increases(input: Vec<u32>) -> u32 {
    let mut window = Vec::new();
    let mut values: Vec<u32> = Vec::new();
    let mut increase_count: u32 = 0;

    for value in input {
        window.push(value);

        if window.len() == 3 {
            values.push(window.iter().sum());
            window.remove(0);
        }

        if values.len() == 2 {
            if values[1] > values[0] {
                increase_count += 1;
            }

            values.remove(0);
        }
    }

    increase_count
}

fn main() {
    let problem_vec = adventlib::string_to_vec::<u32>(INPUT);

    println!(
        "Increase result {}",
        get_value_increases(problem_vec.clone())
    );

    println!(
        "Sliding window result {}",
        get_sliding_window_increases(problem_vec)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_correct_increases() {
        let problem_vec =
            adventlib::string_to_vec::<u32>("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");

        assert_eq!(get_value_increases(problem_vec), 7);
    }

    #[test]
    fn it_should_get_correct_window_increases() {
        let problem_vec =
            adventlib::string_to_vec::<u32>("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");

        assert_eq!(get_sliding_window_increases(problem_vec), 5);
    }
}
