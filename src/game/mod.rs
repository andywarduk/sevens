pub mod play;
pub mod results;
mod state;

use crate::cards::Card;

pub use play::play;
pub use results::Results;
pub use state::State;

const SEVEN_HEARTS: Card = Card::new_from_elems(0, 6);
