/**
 * Team
 * An NCAA tournament team
 * Team includes name, region, and seed.
 */

#[derive(Debug, Default, Clone)]
pub struct Team {
    pub name: String,
    pub region: String,
    pub seed: u8,
}

impl Team {
    /*
    pub fn new(name: &str, region: &str, seed: u8) -> Self {
        Self {
            name: name.to_string(),
            region: region.to_string(),
            seed,
        }
    }
    */
    pub const fn const_default() -> Self {
          Self {
            name: String::new(),
            region: String::new(),
            seed: 0,
          }
    }
}
