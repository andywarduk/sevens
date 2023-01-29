mod card;
mod card_collection;
mod deck;
mod print;

pub use card::Card;
pub use card_collection::CardCollection;
pub use deck::Deck;
pub use print::print_cards;

pub const SUITS: [char; 4] = ['♥', '♣', '♦', '♠'];

pub const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

const SUIT_COLOUR: [&str; 4] = ["red", "black", "red", "black"];

const CARD_HASH: &[u8; 52] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
