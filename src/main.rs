mod bracket;
mod game;
mod round;
mod team;
mod tournament;
mod tournament_field;

use tournament::Tournament;

fn main() {
    let tournament = Tournament::new("NCAA Tournament 2023");
    let my_bracket = tournament.create_bracket("steve bracket");
}
