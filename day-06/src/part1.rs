use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let race_sheets = parse_racesheets(input)?;
    let product: u64 = race_sheets
        .iter()
        .map(|rs| rs.count_winning_strategies())
        .product();
    Ok(product.to_string())
}

pub fn parse_racesheets(input: &str) -> anyhow::Result<Vec<RaceSheet>> {
    let time_parser = preceded(
        tuple((tag("Time:"), space1::<&str, nom::error::Error<&str>>)),
        separated_list1(space1, u64),
    );
    let distance_parser = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, u64),
    );

    let (_, (times, distances)) = separated_pair(time_parser, newline, distance_parser)(input)
        .map_err(|e| anyhow!("Could not parse race sheets: {}", e))?;

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| RaceSheet {
            time: t,
            distance: d,
        })
        .collect_vec())
}

#[derive(Debug, Clone, PartialEq)]
pub struct RaceSheet {
    pub time: u64,
    pub distance: u64,
}

impl RaceSheet {
    pub fn count_winning_strategies(&self) -> u64 {
        (0..=self.time)
            .filter(|ms| self.calculate_distance(ms) > self.distance)
            .count() as u64
    }

    pub fn calculate_distance(&self, button_time: &u64) -> u64 {
        let time_to_travel = self.time - button_time;
        time_to_travel * button_time
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_racesheets() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_equal(
            vec![
                RaceSheet {
                    time: 7,
                    distance: 9,
                },
                RaceSheet {
                    time: 15,
                    distance: 40,
                },
                RaceSheet {
                    time: 30,
                    distance: 200,
                },
            ],
            parse_racesheets(input)?,
        );
        Ok(())
    }

    #[test]
    fn test_calculate_distance() -> anyhow::Result<()> {
        let race_sheet = RaceSheet {
            time: 7,
            distance: 9,
        };
        assert_eq!(0, race_sheet.calculate_distance(&0));
        assert_eq!(6, race_sheet.calculate_distance(&1));
        assert_eq!(10, race_sheet.calculate_distance(&2));
        assert_eq!(12, race_sheet.calculate_distance(&3));
        assert_eq!(12, race_sheet.calculate_distance(&4));
        assert_eq!(10, race_sheet.calculate_distance(&5));
        assert_eq!(6, race_sheet.calculate_distance(&6));
        assert_eq!(0, race_sheet.calculate_distance(&7));
        Ok(())
    }
}
