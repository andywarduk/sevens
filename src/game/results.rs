#[cfg(not(feature = "nostats"))]
use std::cmp::Ordering;

#[cfg(not(feature = "nostats"))]
use crate::cards::CardCollection;
use crate::{numformat::NumFormat, Args};

#[derive(Debug, Default, Clone)]
pub struct PlayerResults {
    pub wins: usize,
    #[cfg(not(feature = "nostats"))]
    pub misses: usize,
    #[cfg(not(feature = "nostats"))]
    pub single: [usize; 4],
    #[cfg(not(feature = "nostats"))]
    pub multi: [usize; 4],
}

impl std::ops::AddAssign<&PlayerResults> for PlayerResults {
    fn add_assign(&mut self, other: &Self) {
        self.wins += other.wins;

        #[cfg(not(feature = "nostats"))]
        {
            self.misses += other.misses;

            other
                .single
                .iter()
                .enumerate()
                .for_each(|(i, v)| self.single[i] += v);

            other
                .multi
                .iter()
                .enumerate()
                .for_each(|(i, v)| self.multi[i] += v);
        }
    }
}

#[derive(Debug)]
pub struct Results {
    player_results: Vec<PlayerResults>,
    games: usize,
}

impl Results {
    pub fn new(players: u8) -> Self {
        Self {
            player_results: vec![PlayerResults::default(); players as usize],
            games: 0,
        }
    }

    #[inline]
    #[cfg(not(feature = "nostats"))]
    pub fn update_stats_for(&mut self, player: usize, cards: &CardCollection, pref_rank: u8) {
        match cards.len().cmp(&1) {
            Ordering::Less => self.player_results[player].misses += 1,
            Ordering::Equal => self.player_results[player].single[pref_rank as usize] += 1,
            Ordering::Greater => self.player_results[player].multi[pref_rank as usize] += 1,
        }
    }

    #[inline]
    pub fn win_for(&mut self, player: usize) {
        self.player_results[player].wins += 1;
        self.games += 1;
    }

    #[inline]
    pub fn games(&self) -> usize {
        self.games
    }

    pub fn player_results(&self) -> &Vec<PlayerResults> {
        &self.player_results
    }

    pub fn add(&mut self, other: Results) {
        self.games += other.games;

        self.player_results
            .iter_mut()
            .zip(other.player_results.iter())
            .for_each(|(a, b)| *a += b);
    }

    pub fn print(&self, args: &Args) {
        println!("Games finished: {}", self.games().num_format());

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

            for (i, player_results) in self.player_results().iter().enumerate() {
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
        let wins = self
            .player_results()
            .iter()
            .map(|w| w.wins.num_format())
            .collect::<Vec<_>>();

        let wins_len = wins.iter().map(|s| s.len()).max().unwrap();

        let pcts = self
            .player_results()
            .iter()
            .map(|w| format!("({:.1}%)", (w.wins as f32 / self.games() as f32) * 100f32))
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
}
