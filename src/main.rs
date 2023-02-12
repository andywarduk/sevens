#![feature(stmt_expr_attributes)]

use std::process::exit;
use std::time::Instant;

use clap::Parser;
use simple_process_stats::ProcessStats;

mod cards;
mod game;
mod numformat;

use crate::{
    cards::{print_cards, Deck},
    game::{play, State, Strategy},
};

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

    /// Strategy
    #[arg(short, long, value_enum, default_value_t = Strategy::Preferred)]
    strategy: Strategy,
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

    print_cards(deck.iter().cloned(), "Card deck:");
    println!("Card deck hash: {}", deck.hash_string());

    // Play
    let state = State::new(args.player_count, deck);

    println!("Player cards:");
    for (i, p) in state.players.iter().enumerate() {
        print_cards(p.card_iterator(), &format!("  Player {}:", i + 1));
    }

    println!("Playing games...");

    let process_stats_start = ProcessStats::get()
        .await
        .expect("could not get stats for running process");
    let start = Instant::now();

    let results = play(state, args.strategy).await;

    let duration = start.elapsed();
    let process_stats_end = ProcessStats::get()
        .await
        .expect("could not get stats for running process");

    println!("Time elapsed: {duration:?}");
    println!(
        "Process time: {:?} user, {:?} kernel",
        process_stats_end.cpu_time_user - process_stats_start.cpu_time_user,
        process_stats_end.cpu_time_kernel - process_stats_start.cpu_time_kernel
    );

    // Print results
    results.print(&args);
}
