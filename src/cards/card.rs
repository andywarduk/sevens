use super::{RANKS, SUITS};

#[derive(Clone, PartialEq)]
pub struct Card(u64);

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:x} ({})", self.0, self))
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.rank(), self.suit()))
    }
}

impl Card {
    pub fn new(suit: &char, rank: &str) -> Option<Self> {
        if let Some(suit) = SUITS.iter().position(|s| s == suit) {
            if let Some(rank) = RANKS.iter().position(|r| *r == rank) {
                return Some(Self::new_from_elems(suit, rank));
            }
        }

        None
    }

    pub fn new_from_elems(suit: usize, rank: usize) -> Self {
        assert!(suit < SUITS.len() && rank < RANKS.len());
        Self(1 << (rank + (suit * 16)))
    }

    pub fn new_from_raw(val: u64) -> Self {
        Self(val)
    }

    pub fn suit(&self) -> char {
        SUITS[self.suit_elem()]
    }

    pub fn rank(&self) -> &str {
        RANKS[self.rank_elem()]
    }

    pub fn suit_elem(&self) -> usize {
        let mut suit = 0;
        let mut shifted = self.0;

        while shifted & 0xffff == 0 {
            shifted >>= 16;
            suit += 1;
        }

        suit
    }

    pub fn rank_elem(&self) -> usize {
        let mut shifted = self.0;

        while shifted & 0xffff == 0 {
            shifted >>= 16;
        }

        15 - ((shifted & 0xffff) as u16).leading_zeros() as usize
    }

    pub fn higher(&self) -> Card {
        Card(self.0 << 1)
    }

    pub fn lower(&self) -> Card {
        Card(self.0 >> 1)
    }

    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for (sno, suit) in SUITS.iter().enumerate() {
            for (rno, rank) in RANKS.iter().enumerate() {
                let card = Card::new(&suit, rank).unwrap();

                assert_eq!(format!("{}{}", rank, suit), format!("{}", card));
                assert_eq!(&card.rank(), rank);
                assert_eq!(&card.suit(), suit);
                assert_eq!(card.rank_elem(), rno);
                assert_eq!(card.suit_elem(), sno);

                let card2 = Card::new_from_elems(sno, rno);
                assert_eq!(card, card2);

                let card3 = Card::new_from_raw(card.raw());
                assert_eq!(card, card3);
            }
        }
    }

    #[test]
    fn test2() {
        for sno in 0..4 {
            for rno in 1..12 {
                let card = Card::new_from_elems(sno, rno);
                let lower = Card::new_from_elems(sno, rno - 1);
                let higher = Card::new_from_elems(sno, rno + 1);

                assert_eq!(card.higher(), higher);
                assert_eq!(card.lower(), lower);
            }
        }
    }
}
