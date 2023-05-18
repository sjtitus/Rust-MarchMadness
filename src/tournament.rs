/*_____________________________________________________________________________
    Tournament
    A tournament is:
       - An array of 64 Team objects that store team details.
       - An array of 63 Game objects that store game state

    NOTE: Teams are explicitly ordered into a tournament by region and seed in
    the same order as specified in module 'tournament_data'.
  _____________________________________________________________________________
*/
use crate::game::Game;
use crate::team::Team;
use crate::tournament_data::get_tournament_data;

/**
 * RegionMatchups
 * How teams in a region are matched up for games by seed.
 * (i.e. 1 plays 16, 8 plays 9, ...)
 */
const REGIONMATCHUPS: [usize; 16] = [1, 16, 8, 9, 5, 12, 4, 13, 6, 11, 3, 14, 7, 10, 2, 15];

#[derive(Debug)]
pub struct Tournament {
    pub name: String,
    // teams and games kept on the heap
    pub teams: Box<[Team; 64]>,
    pub games: Box<[Game; 63]>,
}

impl Tournament {
    // ______________________________________________________________
    // Constructor
    // Year's data must be in tournament data
    pub fn new(year: &str) -> Self {
        const EMPTYTEAM: Team = Team::const_default();
        const EMPTYGAME: Game = Game::const_default();
        let mut s = Tournament {
            name: year.to_string(),
            teams: Box::new([EMPTYTEAM; 64]),
            games: Box::new([EMPTYGAME; 63]),
        };
        s.load_teams()
            .expect(&format!("failed to load tourney data for '{}'", year));
        s.init_games();
        return s;
    }

    /*
        // Create a new tournament bracket
        pub fn create_bracket(&self, bracket_name: &str) -> Bracket {
            let mut b = Bracket::new(bracket_name);
            b.populate(self);
            return b;
        }
    */

    //_________________________________________________________________________
    // private

    // init_first_round_games
    // Initialize the participants in first-round games
    fn init_first_round_games(&mut self) {
        println!("tournament {}: init first round games...", self.name);
        for (gindex, game_ref) in self.games.iter_mut().enumerate() {
            // first round is first 32 games
            if gindex >= 32 {
                break;
            }
            // region_num will be in [0..3]
            let region_num: usize = gindex / 8;
            // index games within a region
            // as if first game is team 0 vs. 1, then 2 vs. 3, etc.
            let reg_gindex: usize = gindex as usize % 8;
            let mut reg_tindex_home = 2 * reg_gindex;
            let mut reg_tindex_away = reg_tindex_home + 1;
            // now use REGIONMATCHUPS to get the real matchup
            // indices that reflect seed: 1 seed vs. 16 seed, etc.
            reg_tindex_home = REGIONMATCHUPS[reg_tindex_home] - 1;
            reg_tindex_away = REGIONMATCHUPS[reg_tindex_away] - 1;
            // shift them to appropriate region
            // (as regions are layed out 0-3)
            let tindex_home = (region_num * 16) + reg_tindex_home;
            let tindex_away = (region_num * 16) + reg_tindex_away;
            // assign the home (higher seed) and away (lower seed)
            // teams as 0 and 1 in the game
            game_ref.teams[0] = tindex_home as u8;
            game_ref.teams[1] = tindex_away as u8;

            /*
            println!(
                "    gindex: {}, region {}: {} ({}) vs {} ({}): next: {}",
                gindex,
                region_num,
                self.teams[tindex_home].name,
                self.teams[tindex_home].seed,
                self.teams[tindex_away].name,
                self.teams[tindex_away].seed,
                game_ref.next.as_ref().unwrap()
            );
            */
        }
    }

    // init_round_prev_next
    // Initialize round, prev, next references for all games
    fn init_round_prev_next(&mut self) {
        println!("tournament {}: init prev/next games...", self.name);
        for (gindex, game_ref) in self.games.iter_mut().enumerate() {
            game_ref.score = [0, 0];
            game_ref.completed = false;
            // round 1: 64 teams playing 32 games 0:31
            if gindex < 32 {
                game_ref.round = 0;
                game_ref.prev[0] = None;
                game_ref.prev[1] = None;
                game_ref.next = Some((32 + gindex / 2) as u8);
            }
            // round 2: 32 teams playing 16 games 32:47
            else if gindex < 48 {
                game_ref.round = 1;
                let p = (2 * (gindex - 32)) as u8;
                game_ref.prev[0] = Some(p);
                game_ref.prev[1] = Some(p + 1);
                game_ref.next = Some((48 + (gindex - 32) / 2) as u8);
            }
            // sweet 16: 16 teams playing 8 games 48:55
            else if gindex < 56 {
                game_ref.round = 2;
                let p = 32 + (2 * (gindex - 48)) as u8;
                game_ref.prev[0] = Some(p);
                game_ref.prev[1] = Some(p + 1);
                game_ref.next = Some((56 + (gindex - 48) / 2) as u8);
            }
            // elite 8: 8 teams playing 4 games 56:59
            else if gindex < 60 {
                game_ref.round = 3;
                let p = 48 + (2 * (gindex - 56)) as u8;
                game_ref.prev[0] = Some(p);
                game_ref.prev[1] = Some(p + 1);
                game_ref.next = Some((60 + (gindex - 56) / 2) as u8);
            }
            // final 4: 4 teams playing 2 games 60:61
            else if gindex < 62 {
                game_ref.round = 4;
                let p = 56 + (2 * (gindex - 60)) as u8;
                game_ref.prev[0] = Some(p);
                game_ref.prev[1] = Some(p + 1);
                game_ref.next = Some((62 + (gindex - 60) / 2) as u8);
            }
            // final game: 2 teams playing 1 game 62
            else {
                game_ref.round = 5;
                let p = 60 + (2 * (gindex - 62)) as u8;
                game_ref.prev[0] = Some(p);
                game_ref.prev[1] = Some(p + 1);
                game_ref.next = None;
            }
        }
    }

    //
    // init_games
    // Initialize the tournament games
    // Assigns teams to first round games, sets up prev/next indices of
    // games to create tournament "tree" structure
    fn init_games(&mut self) {
        self.init_round_prev_next();
        self.init_first_round_games();
    }

    //
    // load_teams
    // Load the tournament teams from an external data source
    fn load_teams(&mut self) -> Result<(), String> {
        println!("Loading teams for tournament '{}'...", self.name);
        // get the data for the given year, fail if we don't have it
        let data = get_tournament_data(&self.name)?;
        // create teams from the tourney data
        for (i, team_tuple) in data.iter().enumerate() {
            /*
            println!(
                "   Team: {} ({} region, {} seed)",
                team_tuple.0, team_tuple.1, team_tuple.2
            );
            */
            let ref_team = &mut (self.teams[i]);
            ref_team.name = team_tuple.0.to_string();
            ref_team.region = team_tuple.1.to_string();
            ref_team.seed = team_tuple.2;
        }
        return Ok(());
    }
}
