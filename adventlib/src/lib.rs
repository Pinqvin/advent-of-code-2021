use std::fmt::Debug;
use std::str::FromStr;

pub fn string_to_vec<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .trim()
        .lines()
        .map(|x| x.trim().parse::<T>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_u32_vector() {
        let input = "123\n456\n789\n";
        assert_eq!(string_to_vec::<u32>(input), vec![123, 456, 789]);
    }
}
