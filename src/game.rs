/*_____________________________________________________________________________
    Game
    A tournament game holds game info
    (teams playing, score, completed, round info).
  _____________________________________________________________________________
*/

#[derive(Debug, Default, Clone, Copy)]
pub struct Game {
    // team specifier = index of team within tournament
    pub teams: [Option<u16>; 2],
    pub score: [u16; 2],
    pub completed: bool,
    pub round: u8,
    pub index: usize,
}
