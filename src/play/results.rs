#[derive(Debug)]
pub struct Results {
    wins: Vec<usize>,
    games: usize,
}

impl Results {
    pub fn new(players: u8) -> Self {
        Self {
            wins: vec![0; players as usize],
            games: 0,
        }
    }

    pub fn win_for(&mut self, player: usize) {
        self.wins[player] += 1;
        self.games += 1;
    }

    #[inline]
    pub fn games(&self) -> usize {
        self.games
    }

    pub fn wins(&self) -> &Vec<usize> {
        &self.wins
    }
}
