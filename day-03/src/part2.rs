use crate::part1::{collect_schematic_parts, collect_schematic_symbols};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let symbols = collect_schematic_symbols(input);
    let schematic_parts = collect_schematic_parts(input);

    let asterisks = symbols.iter().filter(|s| s.char == '*').collect::<Vec<_>>();

    let result: u32 = asterisks
        .into_iter()
        .filter_map(|s| {
            let gear_parts = schematic_parts
                .iter()
                .filter(|p| p.is_engine_part(&[s.to_owned()]))
                .map(|p| p.id as u32)
                .collect::<Vec<_>>();

            match gear_parts.len() {
                2 => Some(gear_parts.iter().product::<u32>()),
                _ => None,
            }
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", solve(input)?);
        Ok(())
    }
}
