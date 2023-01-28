#![feature(portable_simd)]

use std::process::exit;

use clap::Parser;

mod cards;
mod game;
mod numformat;

use game::{play::play, Results, State};

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

    /// Deck hash
    #[arg(short, long)]
    deck_hash: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Create the card deck
    let deck = if let Some(hash) = &args.deck_hash {
        let deck = Deck::new_from_hash(hash);

        if deck.is_none() {
            println!("Card deck hash {hash} is not valid");
            exit(1);
        }

        deck.unwrap()
    } else {
        let mut deck = Deck::new();

        // Shuffle the deck
        if !args.no_shuffle {
            deck.shuffle();
        }

        deck
    };

    println!("Card deck: {deck}");
    println!("Card deck hash: {}", deck.hash_string());

    // Play
    let state = State::new(args.player_count, deck);

    println!("Player cards:");
    for (i, p) in state.players.iter().enumerate() {
        println!("  Player {}: {}", i + 1, p);
    }

    println!("Playing games...");
    let results = play(state).await;

    // Print results
    print_results(&args, results);
}

fn print_results(args: &Args, results: Results) {
    println!("Games finished: {}", results.games().num_format());

    let player_str = (1..=args.player_count)
        .map(|p| format!("{p}"))
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
