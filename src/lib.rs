/*
impl Tournament {
    // constructor
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            team_map: HashMap::new(),
        }
    }
    // add a team to the tourney
    pub fn add_team(&mut self, name: &str) {
        println!("adding team {}", name);
        self.team_map
            .insert(name.to_string(), Team::new(name.to_string()));
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Region {
    East,
    West,
    Midwest,
    South,
}

#[derive(Debug, Clone, Default)]
pub struct RegionBracket {
    pub teams: [Team; 16],
}

*/
