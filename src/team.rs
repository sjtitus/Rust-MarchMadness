/**
 * Team
 * An NCAA tournament team
 *
 * This class encapsulates a team's details as well as results in the context of a
 * tournament: it region, seed, and the games (up to 6) in which it plays.
 *
 */

#[derive(Debug, Default)]
pub struct Team {
    pub name: String,
    pub region: String,
    pub seed: u8,
}

impl Team {
    pub fn new(name: &str, region: &str, seed: u8) -> Self {
        Self {
            name: name.to_string(),
            region: region.to_string(),
            seed,
        }
    }
}
