/**
 * Team
 * An NCAA tournament team
 *
 * This class encapsulates a team's details as well as results in the context of a
 * tournament: it region, seed, and the games (up to 6) in which it plays.
 *
 */
use crate::game::Game;

#[derive(Debug, Default)]
pub struct Team<'t> {
    pub name: String,
    pub region: String,
    pub seed: u8,
    pub games: [Option<&'t Game<'t>>; 6],
}

impl<'t> Team<'t> {
    pub fn new(name: &str, region: &str, seed: u8) -> Self {
        Self {
            name: name.to_string(),
            region: region.to_string(),
            seed,
            games: [None; 6],
        }
    }
}