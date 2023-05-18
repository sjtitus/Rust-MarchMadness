mod game;
mod bracket;
mod team;
mod tournament;
mod tournament_data;

use tournament::Tournament;
use bracket::Bracket;

fn main() {
    let tournament = Tournament::new("2023");
    // let my_bracket = tournament.create_bracket("steve bracket");
    let bracket: Bracket = Bracket::new("steve bracket", &tournament);
    let bracket2: Bracket = Bracket::new("steve bracket2", &tournament);
    println!("Bracket is: {}, tourney: {:p}, games: {:p}", bracket.name, bracket.tournament, bracket.games);
    println!("Bracket2 is: {}, tourney: {:p}, games: {:p}", bracket2.name, bracket2.tournament, bracket2.games);
}
