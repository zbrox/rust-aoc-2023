use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, newline, space0, space1},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let (directions, map) = parse_map(input)?;

    let mut directions_it = directions.iter().cycle();

    let mut current = "AAA";
    let mut step_count = 0;

    while current != "ZZZ" {
        step_count += 1;
        let entry = map.get(current).expect("Dead end");
        let direction = directions_it.next().unwrap();
        current = match direction {
            Direction::Left => &entry.0,
            Direction::Right => &entry.1,
        }
    }

    Ok(step_count.to_string())
}

pub type Map = HashMap<String, (String, String)>;

pub fn parse_map(input: &str) -> anyhow::Result<(Vec<Direction>, Map)> {
    let (rest, directions) =
        directions(input).map_err(|e| anyhow!("Could not parse directions: {}", e))?;
    let (rest, _) = many1(newline::<&str, nom::error::Error<&str>>)(rest)
        .map_err(|e| anyhow!("Missing newline separator: {}", e))?;
    let (_, entries) = many1(terminated(map_entry, opt(line_ending)))(rest)
        .map_err(|e| anyhow!("Could not parse map entries: {}", e))?;

    Ok((directions, entries.into_iter().collect()))
}

pub type MapEntry = (String, (String, String));

fn map_entry(input: &str) -> IResult<&str, MapEntry> {
    separated_pair(
        map(alphanumeric1, |v: &str| v.to_string()),
        tuple((space1, tag("="), space1)),
        delimited(
            tag("("),
            map(
                separated_pair(
                    alphanumeric1::<&str, nom::error::Error<&str>>,
                    tuple((space0, tag(","), space0)),
                    alphanumeric1,
                ),
                |v| (v.0.to_string(), v.1.to_string()),
            ),
            tag(")"),
        ),
    )(input)
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map(alt((tag("L"), tag("R"))), |v| match v {
        "L" => Direction::Left,
        _ => Direction::Right,
    }))(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", solve(input)?);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_directions() -> anyhow::Result<()> {
        let (_, directions) = directions("LRLLRRL")?;
        assert_eq!(
            vec![
                Direction::Left,
                Direction::Right,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
                Direction::Left,
            ],
            directions
        );
        Ok(())
    }

    #[test]
    fn test_parse_map_entry() -> anyhow::Result<()> {
        let (_, entry) = map_entry("PNM = (QGP, BFT)")?;
        assert_eq!(&entry.0, "PNM");
        assert_eq!(&entry.1 .0, "QGP");
        assert_eq!(&entry.1 .1, "BFT");
        Ok(())
    }
}
