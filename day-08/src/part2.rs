use num::integer::lcm;

use itertools::Itertools;

use crate::part1::{parse_map, Direction};

#[tracing::instrument(skip_all)]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let (directions, map) = parse_map(input)?;

    let mut directions_it = directions.iter().cycle();

    let start = map.keys().filter(|v| v.ends_with('A')).collect_vec();
    
    // find each individual path to a Z-suffix
    let mut path_steps = start.iter().map(|v| {
        let mut current = *v;
        let mut step_count = 0;
        while !current.ends_with('Z') {
            step_count += 1;
            let entry = map.get(current).expect("Dead end");
            let direction = directions_it.next().unwrap();
            current = match direction {
                Direction::Left => &entry.0,
                Direction::Right => &entry.1,
            }
        }
        step_count
    });

    let init_lcm: u64 = lcm(path_steps.next().unwrap(), path_steps.next().unwrap());

    // each path is a repeating cycle, find the common multiple to find when they'll sync
    // thanks to this https://www.reddit.com/r/adventofcode/comments/18did3d/2023_day_8_part_1_my_input_maze_plotted_using/
    let lcm = path_steps.fold(init_lcm, |acc, v| {
        lcm(acc, v)
    });

    Ok(lcm.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", solve(input)?);
        Ok(())
    }
}
