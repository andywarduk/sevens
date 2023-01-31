use colored::*;

use super::{CARD_HASH, RANKS, SUITS, SUIT_COLOUR};

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

    pub const fn new_from_elems(suit: usize, rank: usize) -> Self {
        debug_assert!(suit < SUITS.len() && rank < RANKS.len());
        Self(1 << (rank + (suit * 16)))
    }

    pub fn new_from_hash(hash: u8) -> Option<Self> {
        match CARD_HASH.iter().position(|h| *h == hash) {
            Some(p) => {
                let suit = p / 13;
                let rank = p % 13;
                Some(Self::new_from_elems(suit, rank))
            }
            None => None,
        }
    }

    pub const fn new_from_raw(val: u64) -> Self {
        Self(val)
    }

    pub fn suit(&self) -> char {
        SUITS[self.suit_elem() as usize]
    }

    pub fn rank(&self) -> &str {
        RANKS[self.rank_elem() as usize]
    }

    pub fn hash_val(&self) -> char {
        CARD_HASH[((self.suit_elem() as usize * 13) + self.rank_elem() as usize)] as char
    }

    #[inline]
    pub fn suit_elem(&self) -> u32 {
        self.0.trailing_zeros() >> 4
    }

    #[inline]
    pub fn rank_elem(&self) -> u32 {
        self.0.trailing_zeros() & 0xf
    }

    #[inline]
    pub fn one_higher(&self) -> Card {
        Card(self.0 << 1)
    }

    #[inline]
    pub fn one_lower(&self) -> Card {
        Card(self.0 >> 1)
    }

    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }

    pub const CARD_COLOURED_WIDTH: usize = 5;

    pub fn coloured(&self) -> ColoredString {
        format!(" {:<2}{} ", self.rank(), self.suit())
            .color(SUIT_COLOUR[self.suit_elem() as usize])
            .on_bright_white()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for (sno, suit) in SUITS.iter().enumerate() {
            for (rno, rank) in RANKS.iter().enumerate() {
                let card = Card::new(suit, rank).unwrap();

                assert_eq!(format!("{rank}{suit}"), format!("{card}"));
                assert_eq!(&card.rank(), rank);
                assert_eq!(&card.suit(), suit);
                assert_eq!(card.rank_elem(), rno as u32);
                assert_eq!(card.suit_elem(), sno as u32);

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

                assert_eq!(card.one_higher(), higher);
                assert_eq!(card.one_lower(), lower);
            }
        }
    }
}
