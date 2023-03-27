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
    pub tournament_name: String,
    pub rounds: [Option<Round<'b>>; 6],
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
            tournament_name: String::default(),
            rounds: [EMPTYROUND; 6],
        };
        // create empty bracket rounds
        for i in 0..6 {
            s.rounds[i] = Some(Round::<'b>::new(
                format!("round {}", i).to_string(),
                i as u8,
            ));
        }
        // create tree structure by wiring rounds together
        s.connect_rounds();
        return s;
    }
}

impl<'b> Bracket<'b> {
    //_________________________________________________________________________
    // Create a new empty named bracket
    pub fn new(bracket_name: &str) -> Self {
        let mut s = Bracket::default();
        s.name = bracket_name.to_string();
        return s;
    }

    //_________________________________________________________________________
    // populate
    // Populate this bracket for a specific tournament
    // Assigns teams to the round 0 games.
    pub fn populate(&mut self, tournament: &'b Tournament) {
        println!(
            "populating bracket {} using tournament {}",
            self.name, tournament.name
        );

        // Iterate over round 0 games (gindex is [0 to 31])
        let game_vect = self.rounds[0].as_mut().unwrap().games.as_mut().unwrap();
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

            // assign home and away teams
            game.team_home = Some(tournament.teams[tindex_home].as_ref().unwrap());
            game.team_away = Some(tournament.teams[tindex_away].as_ref().unwrap());

            println!(
                "\nBracket {}: round 0 game {}:\n\thome: {}\n\taway: {}\n\tprev: {}/{}",
                self.name,
                gindex,
                game.team_home.unwrap().name,
                game.team_away.unwrap().name,
                game.prev[0].is_none(),
                game.prev[1].is_none()
            );
        }
    }

    // _____________________ private _______________________
    //

    // connect_rounds
    // connect a bracket's rounds into tournament "tree" structure
    fn connect_rounds(&mut self) {
        // Iterate through rounds 1-5, setting 'prev' pointers for current
        // round and 'next' pointers for previous round
        for rindex in 1..6 {
            //
            // get mutable refs to current and previous rounds (rindex and rindex-1).
            // Note: we can safely unwrap() because we've populated everything.
            let (prev_array, current_array) = self.rounds.split_at_mut(rindex);
            let prev_round = prev_array[prev_array.len() - 1].as_mut().unwrap();
            let prev_games = prev_round.games.as_mut().unwrap();
            let current_round = current_array[0].as_mut().unwrap();
            let current_games = current_round.games.as_mut().unwrap();

            for (gindex, cg) in current_games.iter_mut().enumerate() {
                let first: usize = 2 * gindex;
                let second: usize = first + 1;
                // println!("\nconnect: round {} game {}:\n\tprev: round {} games {} and {}",
                //    rindex, gindex, rindex - 1, first, second);
                (*cg).prev[0] = Some(first);
                (*cg).prev[1] = Some(second);
                // println!("connect: \tround {} games {} and {}: next game: round {} game {}",
                //    rindex - 1, first, second, rindex, gindex);
                prev_games[first].next = Some(gindex);
                prev_games[second].next = Some(gindex);
            }
        }
    }
}
