use crate::cards::CardCollection;

#[derive(Debug, Clone)]
pub struct Player {
    pub cards: CardCollection,
}

impl Player {
    pub fn new() -> Self {
        Self {
            cards: CardCollection::new(),
        }
    }
}
