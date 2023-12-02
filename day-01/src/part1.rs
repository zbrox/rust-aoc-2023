#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let sum: u32 = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|line| {
            format!(
                "{}{}",
                line.chars().next().unwrap(),
                line.chars().next_back().unwrap()
            )
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", solve(input)?);
        Ok(())
    }
}
