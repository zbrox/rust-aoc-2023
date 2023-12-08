use std::{collections::HashMap, ops::Range};

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, newline, space0, space1, u64},
    combinator::opt,
    multi::{fold_many0, fold_many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let almanac = parse_almanac(input)?;
    tracing::info!("Parsed the almanac");

    let mut from = "seed";
    let target = "location";
    let mut buff: Vec<u64> = almanac.seeds;

    while from != target {
        match almanac.maps.get(from) {
            None => break,
            Some(map) => {
                println!("from/to {} {}", from, map.to);
                from = &map.to;
                buff = buff
                    .iter()
                    .map(|num| {
                        map.map
                            .iter()
                            // .filter(|(src, dest)| )
                            .find_map(|(src, dest)| match src.contains(num) {
                                false => None,
                                true => {
                                    let pos = num - src.start;
                                    Some(dest.start + pos)
                                }
                            })
                            .unwrap_or(*num)
                    })
                    .collect();
            }
        }
    }

    Ok(buff.iter().min().unwrap_or(&0u64).to_string())
}

#[derive(Debug)]
pub struct AlmanacMap {
    pub from: String,
    pub to: String,
    pub map: Vec<(Range<u64>, Range<u64>)>,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: HashMap<String, AlmanacMap>,
}

pub fn parse_almanac(input: &str) -> anyhow::Result<Almanac> {
    let (rest, seeds) = terminated(parse_seeds, newline)(input)
        .map_err(|e| anyhow!("Failed to parse seeds: {}", e))?;
    let (rest, _) = newline::<_, nom::error::Error<&str>>(rest)
        .map_err(|_| anyhow!("Missing newline separator"))?;
    let (_, maps) = fold_many1(
        terminated(parse_almanac_map, opt(newline)),
        HashMap::new,
        |mut acc, v| {
            acc.insert(v.from.to_string(), v);
            acc
        },
    )(rest)
    .map_err(|e| anyhow!("Failed to parse almanac maps: {}", e))?;

    Ok(Almanac { seeds, maps })
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tuple((tag("seeds:"), space1)), separated_list1(space1, u64))(input)
}

pub fn parse_almanac_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (rest, (from_name, to_name)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((space1, tag("map:"), line_ending)),
    )(input)?;

    let range_map_parser = terminated(
        tuple((
            terminated(u64::<_, nom::error::Error<&str>>, space1),
            terminated(u64, space1),
            terminated(u64, space0),
        )),
        opt(line_ending),
    );

    let (rest, map) = fold_many0(
        range_map_parser,
        Vec::new,
        |mut acc, (dest_start, orig_start, len)| {
            acc.push((orig_start..orig_start + len, dest_start..dest_start + len));
            acc
        },
    )(rest)?;

    Ok((
        rest,
        AlmanacMap {
            from: from_name.to_string(),
            to: to_name.to_string(),
            map,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_seeds() {
        let (_, res) = parse_seeds("seeds: 79 14 55 13").unwrap();
        assert_eq!(vec![79, 14, 55, 13], res);
    }

    #[test]
    fn test_parse_almanac_map() {
        let (_, res) = parse_almanac_map(
            "temperature-to-humidity map:
0 69 1
1 0 69",
        )
        .unwrap();
        assert_eq!("temperature", res.from);
        assert_eq!("humidity", res.to);
        assert_eq!(vec![(69..70, 0..1), (0..69, 1..70),], res.map);
    }
}
