use itertools::put_back_n;

use crate::part1::{parse_card, Card};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let cards: Vec<Card> = input.lines().flat_map(parse_card).collect();
    let mut new_cards_count: usize = 0;

    let mut orig_cards_it = put_back_n(cards.iter());

    // ヽ(￣(ｴ)￣)ﾉ
    // very slow
    while let Some(c) = orig_cards_it.next() {
        let num_new_cards = c.matches().len();
        new_cards_count += num_new_cards;
        cards
            .iter()
            .skip(c.id as usize)
            .take(num_new_cards)
            .for_each(|new_card| {
                orig_cards_it.put_back(new_card)
            });
    }

    Ok((new_cards_count + cards.len()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", solve(input)?);
        Ok(())
    }
}
