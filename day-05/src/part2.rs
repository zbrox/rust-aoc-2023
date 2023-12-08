use std::{collections::HashMap, ops::Range};

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::{map, opt},
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rangetools::Rangetools;

use crate::part1::{parse_almanac_map, AlmanacMap};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let almanac = parse_almanac(input)?;
    tracing::info!("Parsed the almanac");

    let mut from = "seed";
    let target = "location";
    let mut buff: Vec<Range<u64>> = almanac.seeds;

    while from != target {
        match almanac.maps.get(from) {
            None => break,
            Some(map) => {
                from = &map.to;
                buff = buff
                    .iter()
                    .flat_map(|num_range| {
                        let new_dest_to_buff_mapped = map
                            .map
                            .iter()
                            .filter_map(|(src, dest)| {
                                let intersection: Range<u64> =
                                    match src.clone().intersects(num_range.clone()) {
                                        true => num_range.clone().intersection(src.clone()).into(),
                                        false => return None,
                                    };
                                let intersection_offset = intersection.start - src.start;
                                let intersection_len = intersection.end - intersection.start;

                                let dest_intersection = dest.start + intersection_offset
                                    ..dest.start + intersection_offset + intersection_len - 1;

                                Some(dest_intersection)
                            })
                            .collect::<Vec<Range<u64>>>();

                        match new_dest_to_buff_mapped.is_empty() {
                            true => vec![num_range.clone()],
                            false => new_dest_to_buff_mapped,
                        }
                    })
                    .collect();
            }
        }
    }

    let min = buff.iter().map(|v| v.start).min().unwrap_or(0);

    Ok(min.to_string())
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<Range<u64>>,
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

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let range_parser = map(separated_pair(u64, space1, u64), |(start, end)| {
        start..start + end
    });
    preceded(
        tuple((tag("seeds:"), space1)),
        separated_list1(space1, range_parser),
    )(input)
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
        assert_eq!("46", solve(input)?);
        Ok(())
    }
}
