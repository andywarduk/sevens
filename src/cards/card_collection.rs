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

    pub fn iter(&self) -> CardCollectionIterator {
        CardCollectionIterator(self.0)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl std::fmt::Display for CardCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, c) in self.iter().enumerate() {
            if i > 0 {
                string += " ";
            }

            string += &format!("{}", c);
        }

        f.write_str(&string)
    }
}

pub struct CardCollectionIterator(u64);

impl Iterator for CardCollectionIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.0.trailing_zeros();

        if next_pos == 64 {
            None
        } else {
            let next = 1 << next_pos;
            self.0 &= !next;
            Some(Card::new_from_raw(next))
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

        let mut deck = Deck::new();

        while let Some(card) = deck.pop() {
            if card == card1 || card == card2 || card == card3 || card == card4 {
                assert_eq!(true, collection.contains(card.clone()));
            } else {
                assert_eq!(false, collection.contains(card.clone()));
            }
        }

        // Test iterating cards
        let mut iter = collection.iter();

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
                assert_eq!(true, collection.contains(card.clone()));
            } else {
                assert_eq!(false, collection.contains(card.clone()));
            }
        }

        // Test removing all cards
        collection.remove(card3);
        collection.remove(card4);

        let mut deck = Deck::new();

        while let Some(card) = deck.pop() {
            assert_eq!(false, collection.contains(card.clone()));
        }
    }
}
