use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub const SIZE: usize = 2020;

pub struct Day15VersionA;

impl DaySolver for Day15VersionA {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file_and_variant(
        "day_15",
        "data_files/ex15.txt",
        "vector with tuple",
    );

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut turns: Vec<(usize, u32)> = _s
            .split(',')
            .enumerate()
            .map(|(turn_num, value)| (turn_num + 1, value.parse::<u32>().unwrap()))
            .collect();

        let (_, start_value) = turns.pop().unwrap();

        let res = (turns.len() + 1..SIZE).fold(start_value, |new_num, turn_num| {
            match turns.iter().position(|&(_, num)| num == new_num) {
                None => {
                    turns.push((turn_num, new_num));
                    0
                },
                Some(index) => {
                    let next_value = (turn_num - turns[index].0) as u32;
                    turns[index] = (turn_num, new_num);
                    next_value
                },
            }
        });

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0,3,6" => 436)]
    #[test_case("1,3,2" => 1)]
    #[test_case("2,1,3" => 10)]
    #[test_case("1,2,3" => 27)]
    #[test_case("2,3,1" => 78)]
    #[test_case("3,2,1" => 438)]
    #[test_case("3,1,2" => 1836)]
    fn data_from_exapmles(s: &str) -> u32 {
        Day15VersionA::solution(s).unwrap()
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day15VersionA::solve_default_file().unwrap(), 436)
    }
}
