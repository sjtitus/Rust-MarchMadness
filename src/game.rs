/*_____________________________________________________________________________
    Game

    A tournament game holds game info (teams playing, round, score, completed),
    and also refers to the previous games that populated it and the
    next game that it feeds (via indices of those games within the previous
    and next rounds).
  _____________________________________________________________________________
*/
use crate::team::Team;

#[derive(Debug, Default, Clone, Copy)]
pub struct Game<'g> {
    pub team_home: Option<&'g Team<'g>>,
    pub team_away: Option<&'g Team<'g>>,
    pub score_home: u16,
    pub score_away: u16,
    pub completed: bool,
    pub round: u8,
    // prev and next are indexes of games within a round
    pub prev: [ Option<usize>; 2 ],
    pub next: Option<usize>
}