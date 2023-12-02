use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, opt, peek},
    multi::{many1, many_till, separated_list0},
    sequence::preceded,
    IResult,
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let sum: u32 = input
        .lines()
        .filter_map(|line| parse_line_digits(line).ok())
        .map(|line| get_calibration_values(&line.1))
        .sum();
    Ok(sum.to_string())
}

fn get_calibration_values(digits: &str) -> u32 {
    format!(
        "{}{}",
        digits.chars().next().unwrap(),
        digits.chars().next_back().unwrap()
    )
    .parse()
    .unwrap()
}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    alt((
        map(tag("one"), |_| "1"),
        map(tag("two"), |_| "2"),
        map(tag("three"), |_| "3"),
        map(tag("four"), |_| "4"),
        map(tag("five"), |_| "5"),
        map(tag("six"), |_| "6"),
        map(tag("seven"), |_| "7"),
        map(tag("eight"), |_| "8"),
        map(tag("nine"), |_| "9"),
        // map(tag("zero"), |_| "0"),
        digit1,
    ))(input)
}

fn parse_not_digit(input: &str) -> IResult<&str, &str> {
    map(many_till(anychar, peek(parse_digit)), |_| "")(input)
}

fn parse_line_digits(input: &str) -> IResult<&str, String> {
    map(
        preceded(
            opt(parse_not_digit),
            map(separated_list0(parse_not_digit, many1(parse_digit)), |v| {
                v.into_iter().flatten().collect::<Vec<_>>()
            }),
        ),
        |v| v.join(""),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_digit_words() {
        let result = parse_digit("eightwothree").unwrap();
        assert_eq!(("wothree", "8"), result);
    }

    #[test]
    fn test_neg_parse_digit_words() {
        let result = parse_not_digit("wothree").unwrap();
        assert_eq!(("three", ""), result);
    }

    #[test]
    fn test_parse_line_digits() {
        let result = parse_line_digits("eightwothree").unwrap();
        assert_eq!(("", "83".to_string()), result);
        let result = parse_line_digits("abcone2threexyz").unwrap();
        assert_eq!(("xyz", "123".to_string()), result);
        let result = parse_line_digits("fkpsxsmchn3ninesevenseventfxxjdnqxtwo").unwrap();
        assert_eq!(("", "39772".to_string()), result);
    }

    #[test]
    fn test_get_calibration_values() {
        assert_eq!(13, get_calibration_values("123"));
        assert_eq!(83, get_calibration_values("83"));
        assert_eq!(13, get_calibration_values("123"));
        assert_eq!(32, get_calibration_values("39772"));
    }
}
