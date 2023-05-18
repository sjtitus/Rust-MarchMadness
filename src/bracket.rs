/*_____________________________________________________________________________
    Bracket
  _____________________________________________________________________________
*/
use crate::tournament::Tournament;
use crate::game::Game;

/**
 * Bracket
 * A bracket is named set of 63 pickable games associated with
 * a specific tournament.
 */
#[derive(Debug)]
pub struct Bracket<'b> {
    pub name: String,
    pub tournament: &'b Tournament,
    pub games: Box<[Game; 63]>,
}

impl<'b> Bracket<'b> {
    //_________________________________________________________________________
    // Constructor
    pub fn new(name: &str, tournament: &'b Tournament) -> Self {
        let s = Bracket {
            name: name.to_string(),
            tournament: tournament,
            games: tournament.games.clone()
        };
        return s;
    }
}
