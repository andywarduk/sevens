mod play;
mod results;
mod state;
mod strategy;

use crate::cards::Card;

pub use play::play;
pub use results::Results;
pub use state::State;
pub use strategy::Strategy;

const SEVEN_HEARTS: Card = Card::new_from_elems(0, 6);
