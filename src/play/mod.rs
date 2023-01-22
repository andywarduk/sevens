mod play;
mod player;
mod results;
mod state;

use lazy_static::lazy_static;

use crate::cards::Card;

pub use play::play;
pub use results::Results;
pub use state::State;

lazy_static! {
    static ref SEVEN_HEARTS: Card = Card::new(&'♥', "7").unwrap();
}
