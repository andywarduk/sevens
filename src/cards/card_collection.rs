use super::Card;

#[derive(Debug, Clone)]
pub struct CardCollection(u64);

impl CardCollection {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn new_single(card: Card) -> Self {
        Self(card.raw())
    }

    pub fn new_from_raw(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn add(&mut self, card: Card) {
        self.0 |= card.raw();
    }

    #[inline]
    pub fn remove(&mut self, card: Card) {
        self.0 &= !card.raw();
    }

    #[inline]
    pub fn contains(&self, card: Card) -> bool {
        self.0 & card.raw() != 0
    }

    #[inline]
    pub fn contains_one_to_six_except(&self, suit: u32, card: &Card) -> bool {
        self.0 & !card.raw() & (0x3f << (suit * 16)) != 0
    }

    #[inline]
    pub fn contains_eight_to_king_except(&self, suit: u32, card: &Card) -> bool {
        self.0 & !card.raw() & (0x1f80 << (suit * 16)) != 0
    }

    #[inline]
    pub fn first(&self) -> u64 {
        // Get the least significant bit
        let first = (self.0 as i64) & -(self.0 as i64);
        first as u64
    }

    pub fn card_iterator(&self) -> CardCollectionIterator {
        CardCollectionIterator(self.0 as i64)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for CardCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, c) in self.card_iterator().enumerate() {
            if i > 0 {
                string += " ";
            }

            string += &format!("{c}");
        }

        f.write_str(&string)
    }
}

pub struct CardCollectionIterator(i64);

impl Iterator for CardCollectionIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            // Get the least significant bit
            let next = self.0 & -self.0;

            // Remove from self
            self.0 &= !next;

            // Return card
            Some(Card::new_from_raw(next as u64))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Deck;

    use super::*;

    #[test]
    fn test1() {
        let mut collection = CardCollection::new();

        let card1 = Card::new(&'♥', "A").unwrap();
        let card2 = Card::new(&'♣', "7").unwrap();
        let card3 = Card::new(&'♦', "10").unwrap();
        let card4 = Card::new(&'♠', "K").unwrap();

        // Test adding cards
        collection.add(card1.clone());
        collection.add(card2.clone());
        collection.add(card3.clone());
        collection.add(card4.clone());

        // Test contains A-6 and 8-K
        println!("{:x} {:x}", collection.raw(), card2.raw());
        assert!(collection.contains_one_to_six_except(0, &card2));
        assert!(!collection.contains_eight_to_king_except(0, &card2));
        assert!(!collection.contains_one_to_six_except(1, &card2));
        assert!(!collection.contains_eight_to_king_except(1, &card2));
        assert!(!collection.contains_one_to_six_except(2, &card2));
        assert!(collection.contains_eight_to_king_except(2, &card2));
        assert!(!collection.contains_one_to_six_except(3, &card2));
        assert!(collection.contains_eight_to_king_except(3, &card2));

        // Test contains
        let mut deck = Deck::new();

        while let Some(card) = deck.pop() {
            if card == card1 || card == card2 || card == card3 || card == card4 {
                assert!(collection.contains(card.clone()));
            } else {
                assert!(!collection.contains(card.clone()));
            }
        }

        // Test iterating cards
        let mut iter = collection.card_iterator();

        assert_eq!(iter.next(), Some(card1.clone()));
        assert_eq!(iter.next(), Some(card2.clone()));
        assert_eq!(iter.next(), Some(card3.clone()));
        assert_eq!(iter.next(), Some(card4.clone()));
        assert_eq!(iter.next(), None);

        // Test removing some cards
        collection.remove(card1);
        collection.remove(card2);

        let mut deck = Deck::new();

        while let Some(card) = deck.pop() {
            if card == card3 || card == card4 {
                assert!(collection.contains(card.clone()));
            } else {
                assert!(!collection.contains(card.clone()));
            }
        }

        // Test removing all cards
        collection.remove(card3);
        collection.remove(card4);

        let mut deck = Deck::new();

        while let Some(card) = deck.pop() {
            assert!(!collection.contains(card.clone()));
        }
    }
}
