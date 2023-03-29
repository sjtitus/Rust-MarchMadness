/*_____________________________________________________________________________
    Bracket
  _____________________________________________________________________________
*/
use crate::round::Round;
use crate::tournament::Tournament;
use crate::tournament_field::REGIONMATCHUPS;

/**
 * Bracket
 * A bracket is six rounds of games organized into a tree
 * structure (first 2 games of round 0 feed first game of round 1,
 * etc). A bracket has a name and is associated with a specific
 * tournament.
 */
#[derive(Debug)]
pub struct Bracket<'b> {
    pub name: String,
    pub tournament: Option<&'b Tournament>,
    pub rounds: [Option<Round>; 6],
}

/**
 * Default Bracket
 * Empty bracket: empty name, no tournament association;
 * i.e. games in round 0 do not have teams.
 */
impl<'b> Default for Bracket<'b> {
    fn default() -> Self {
        const EMPTYROUND: Option<Round> = None;
        let mut s = Bracket {
            name: String::default(),
            tournament: None,
            rounds: [EMPTYROUND; 6],
        };
        // create empty bracket rounds
        for i in 0..6 {
            s.rounds[i] = Some(Round::new(
                format!("round {}", i).to_string(),
                i as u8,
            ));
        }
        return s;
    }
}

impl<'b> Bracket<'b> {
    //_________________________________________________________________________
    // Create a new empty named bracket
    pub fn new(bracket_name: &str, tournament: &'b Tournament) -> Self {
        let mut s = Bracket::<'b>::default();
        s.name = bracket_name.to_string();
        s.tournament = Some(tournament);
        s.populate();
        return s;
    }

    //_________________________________________________________________________
    // populate
    // Populate this bracket for a specific tournament
    // Assigns teams to the round 0 games.
    fn populate(&mut self) {
        println!(
            "populating bracket {} using tournament {}",
            self.name, self.tournament.unwrap().name
        );

        // Iterate over round 0 games (gindex is [0 to 31])
        let game_vect = &mut self.rounds[0].as_mut().unwrap().games;
        for (gindex, game) in game_vect.iter_mut().enumerate() {

            // region_num will be [0, 3]
            let region_num: usize = gindex / 8;

            // get the index of the teams in the region
            // NOTE: twizzle teams according to the seed-based
            // matchups (i.e. 1 plays 16)
            let reg_gindex: usize = gindex as usize % 8;
            let mut reg_tindex_home = 2 * reg_gindex;
            let mut reg_tindex_away = reg_tindex_home + 1;
            reg_tindex_home = REGIONMATCHUPS[reg_tindex_home] - 1;
            reg_tindex_away = REGIONMATCHUPS[reg_tindex_away] - 1;

            let tindex_home = (region_num * 16) + reg_tindex_home;
            let tindex_away = (region_num * 16) + reg_tindex_away;

            // populate teams
            game.teams[0] = Some(tindex_home as u16);
            game.teams[1] = Some(tindex_away as u16);

            println!(
                "\nBracket {}: round 0 game {}:\n\thome: {}\n\taway: {}",
                self.name,
                gindex,
                self.tournament.unwrap().teams[tindex_home].as_ref().unwrap().name,
                self.tournament.unwrap().teams[tindex_away].as_ref().unwrap().name,
            );
        }

    }
}
