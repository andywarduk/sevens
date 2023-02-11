#[cfg(not(feature = "nostats"))]
use std::cmp::Ordering;

#[cfg(not(feature = "nostats"))]
use crate::cards::CardCollection;

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
}
