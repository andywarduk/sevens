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
    game::{play, Results, State, Strategy},
    numformat::NumFormat,
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
    print_results(&args, results);
}

fn print_results(args: &Args, results: Results) {
    println!("Games finished: {}", results.games().num_format());

    let player_str = (1..=args.player_count)
        .map(|p| format!("{p}"))
        .collect::<Vec<_>>();

    let player_str_len = player_str.iter().map(|s| s.len()).max().unwrap();

    #[cfg(not(feature = "nostats"))]
    {
        // Print plays
        print!("Plays    {:<player_str_len$}:", "");

        for i in 0..=args.strategy.max_pref_rank() {
            print!(" {:>12} >1", args.strategy.pref_rank_desc(i));
            print!(" {:>13} 1", args.strategy.pref_rank_desc(i));
        }
        println!(" {:>15}", "Missed Goes");

        for (i, player_results) in results.player_results().iter().enumerate() {
            print!("  Player {:<player_str_len$}:", player_str[i]);

            for i in 0..=args.strategy.max_pref_rank() {
                print!(
                    " {:>15} {:>15}",
                    player_results.multi[i as usize].num_format(),
                    player_results.single[i as usize].num_format()
                );
            }

            println!(" {:>15}", player_results.misses.num_format());
        }
    }

    // Print wins
    let wins = results
        .player_results()
        .iter()
        .map(|w| w.wins.num_format())
        .collect::<Vec<_>>();

    let wins_len = wins.iter().map(|s| s.len()).max().unwrap();

    let pcts = results
        .player_results()
        .iter()
        .map(|w| {
            format!(
                "({:.1}%)",
                (w.wins as f32 / results.games() as f32) * 100f32
            )
        })
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
