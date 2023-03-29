/*_____________________________________________________________________________
    Tournament

    A tournament is an array of 64 Team objects that hold team details.

    Teams are explicitly ordered into a tournament by region and seed in
    the same order as specified in tournament_field.
  _____________________________________________________________________________
*/
use crate::bracket::Bracket;
use crate::team::Team;
use crate::tournament_field::get_tournament_field;

#[derive(Debug)]
pub struct Tournament {
    pub name: String,
    // Put array of 64 teams on the heap
    pub teams: Box<[Option<Team>; 64]>,
}

impl Tournament {
    //
    // Create a tournament
    // Name must match the name of an existing tournament field
    pub fn new(name: &str) -> Self {
        const EMPTYTEAM: Option<Team> = None;
        let mut s = Tournament {
            name: name.to_string(),
            teams: Box::new([EMPTYTEAM; 64])
        };
        s.load()
            .expect(&format!("failed to load tourney field for '{}'", name));
        return s;
    }

    // Create a new tournament bracket
    pub fn create_bracket(&self, bracket_name: &str) -> Bracket {
        return Bracket::new(bracket_name, self);
    }

    //_________________________________________________________________________
    // private

    //
    // load
    // Load the tournament teams from an external data source that holds
    // the field for that year. NOTE: currently, the fields are just kept
    // as static data in module tournament_fields.
    fn load(&mut self) -> Result<(), String> {
        println!("Loading tournament '{}'...", self.name);
        // get the field for the given year, fail if we don't have it
        let field = get_tournament_field(&self.name)?;
        // create teams from the tourney field
        for (i, team_tuple) in field.iter().enumerate() {
            println!(
                "   Team: {} ({} region, {} seed)",
                team_tuple.0, team_tuple.1, team_tuple.2
            );
            self.teams[i] = Some(Team::new(team_tuple.0, team_tuple.1, team_tuple.2));
        }
        return Ok(());
    }
}
