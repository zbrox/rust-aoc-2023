use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

use crate::part1::RaceSheet;

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let race_sheet = parse_racesheet(input)?;
    Ok(race_sheet.count_winning_strategies().to_string())
}

pub fn parse_racesheet(input: &str) -> anyhow::Result<RaceSheet> {
    let time_parser = preceded(
        tuple((tag("Time:"), space1::<&str, nom::error::Error<&str>>)),
        map(separated_list1(space1, alphanumeric1), |v| {
            v.join("").parse::<u64>().unwrap()
        }),
    );
    let distance_parser = preceded(
        tuple((tag("Distance:"), space1::<&str, nom::error::Error<&str>>)),
        map(separated_list1(space1, alphanumeric1), |v| {
            v.join("").parse::<u64>().unwrap()
        }),
    );

    let (_, (time, distance)) = separated_pair(time_parser, newline, distance_parser)(input)
        .map_err(|e| anyhow!("Could not parse race sheets: {}", e))?;

    Ok(RaceSheet { time, distance })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Time:      71530
Distance:  940200";
        assert_eq!("71503", solve(input)?);
        Ok(())
    }
}
