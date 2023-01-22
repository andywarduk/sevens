#![feature(portable_simd)]

use clap::Parser;

mod cards;
mod numformat;
mod play;

use play::{play, State};

use crate::{cards::Deck, numformat::NumFormat, play::Results};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of players
    #[arg(short, long = "players", value_parser = clap::value_parser!(u8).range(2..=52), default_value_t = 6)]
    player_count: u8,

    /// Don't shuffle the cards
    #[arg(short, long)]
    no_shuffle: bool,

    /// Debug
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Create the card deck
    let mut deck = Deck::new();

    // Shuffle the deck
    if !args.no_shuffle {
        deck.shuffle();
    }

    println!("Card deck: {}", deck);

    // Play
    let state = State::new(args.player_count, deck);

    println!("Player cards:");
    for (i, p) in state.players.iter().enumerate() {
        println!("  Player {}: {}", i + 1, p.cards);
    }

    println!("Playing games...");
    let mut results = Results::new(args.player_count);

    play(args.debug, state, &mut results);

    println!("Games finished: {}", results.games().num_format());
    println!("Wins:");
    for (i, wins) in results.wins().iter().enumerate() {
        println!("  Player {}: {}", i + 1, wins.num_format());
    }
}
