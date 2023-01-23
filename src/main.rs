#![feature(portable_simd)]

use clap::Parser;

mod cards;
mod numformat;
mod play;

use play::{play, Results, State};

use crate::{cards::Deck, numformat::NumFormat};

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

#[tokio::main]
async fn main() {
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
    let results = play(args.debug, state).await;

    // Print results
    print_results(&args, results);
}

fn print_results(args: &Args, results: Results) {
    println!("Games finished: {}", results.games().num_format());

    let player_str = (1..=args.player_count)
        .map(|p| format!("{}", p))
        .collect::<Vec<_>>();

    let player_str_len = player_str.iter().map(|s| s.len()).max().unwrap();

    let wins = results
        .wins()
        .iter()
        .map(|w| w.num_format())
        .collect::<Vec<_>>();

    let wins_len = wins.iter().map(|s| s.len()).max().unwrap();

    let pcts = results
        .wins()
        .iter()
        .map(|w| format!("({:.1}%)", (*w as f32 / results.games() as f32) * 100f32))
        .collect::<Vec<_>>();

    let pcts_len = pcts.iter().map(|s| s.len()).max().unwrap();

    println!("Wins:");
    for i in 0..args.player_count as usize {
        println!(
            "  Player {:<player_str_len$}: {:>wins_len$} {:>pcts_len$}",
            player_str[i], wins[i], pcts[i]
        );
    }
}
