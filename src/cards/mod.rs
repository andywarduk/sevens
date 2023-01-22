mod card;
mod card_collection;
mod deck;

pub use card::Card;
pub use card_collection::CardCollection;
pub use deck::Deck;

pub const SUITS: [char; 4] = ['♥', '♣', '♦', '♠'];

pub const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];
