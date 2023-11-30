#[tracing::instrument]
pub fn solve(
    _input: &str,
) -> anyhow::Result<String> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "";
        assert_eq!("", solve(input)?);
        Ok(())
    }
}