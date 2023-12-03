use regex::Regex;

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let symbols = collect_schematic_symbols(input);
    let schematic_parts = collect_schematic_parts(input);

    let sum: u32 = schematic_parts
        .iter()
        .filter(|v| v.is_engine_part(&symbols))
        .map(|p| p.id as u32)
        .sum();

    Ok(sum.to_string())
}

#[derive(Debug, Clone)]
pub struct SchematicSymbol {
    pub char: char,
    pub row_num: u16,
    pub pos: u16,
}

pub struct SchematicPart {
    pub id: u16,
    pub row_num: u16,
    pub start_pos: u16,
    pub end_pos: u16,
}

impl SchematicPart {
    pub fn is_engine_part(&self, symbol_coords: &[SchematicSymbol]) -> bool {
        symbol_coords.iter().any(|symbol| {
            if self.row_num.abs_diff(symbol.row_num) > 1 {
                return false;
            }
            let start_pos = match self.start_pos == 0 {
                true => self.start_pos,
                false => self.start_pos - 1,
            };
            (start_pos..=self.end_pos).contains(&symbol.pos)
        })
    }
}

pub fn collect_schematic_symbols(input: &str) -> Vec<SchematicSymbol> {
    input
        .replace('.', " ")
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            line.chars()
                .enumerate()
                .filter(|(_pos, c)| !c.is_alphanumeric() && !c.is_whitespace())
                .map(|(pos, char)| SchematicSymbol {
                    row_num: row_num as u16,
                    pos: pos as u16,
                    char,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn collect_schematic_parts(input: &str) -> Vec<SchematicPart> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            re.find_iter(line)
                .map(|m| SchematicPart {
                    id: m.as_str().parse().unwrap(),
                    row_num: row_num as u16,
                    start_pos: m.start() as u16,
                    end_pos: m.end() as u16,
                })
                .collect::<Vec<SchematicPart>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

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
        assert_eq!("4361", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_collect_symbol_coords() {
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
        let symbols = collect_schematic_symbols(input);
        assert_equal(
            vec![(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)],
            symbols
                .iter()
                .map(|s| (s.row_num, s.pos))
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_collect_schematic_parts() {
        let res = collect_schematic_parts("..35..633.");
        assert_equal(
            vec![35, 633],
            res.iter().map(|p| p.id).collect::<Vec<u16>>(),
        );
        assert_equal(
            vec![2, 6],
            res.iter().map(|p| p.start_pos).collect::<Vec<u16>>(),
        );
        assert_equal(
            vec![4, 9],
            res.iter().map(|p| p.end_pos).collect::<Vec<u16>>(),
        );
        assert_equal(
            vec![0, 0],
            res.iter().map(|p| p.row_num).collect::<Vec<u16>>(),
        );
    }
}
