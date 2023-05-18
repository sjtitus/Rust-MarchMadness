/*_____________________________________________________________________________
    Game
    A tournament game holds game info (teams playing, round, score, completed),
    and also refers to the previous games that populated it and the
    next game that it feeds (via indices of those games within the
    tournament's array of games).
  _____________________________________________________________________________
*/

#[derive(Debug, Default, Clone, Copy)]
pub struct Game {
    // teams: indices into tournament's teams array
    pub teams: [u8; 2],
    pub score: [u8; 2],
    pub completed: bool,
    pub round: u8,
    // prev/next: indices into tournament's games array
    pub prev: [ Option<u8>; 2 ],
    pub next: Option<u8>
}

impl Game {

    pub const fn const_default() -> Self {
        Self {
            teams: [0, 0],
            score: [0, 0],
            completed: false,
            round: 0,
            prev: [ None, None ],
            next: None
        }
    }
}
