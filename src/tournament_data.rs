/*_____________________________________________________________________________
    TournamentData: NCAA Tournament Data

    Static heap-resident data structure currently used as the 'external'
    store of men's NCAA tourney data by year. Contains the following info
    about each team in the tourney: name, region (east, west, south, midwest)
    and seed (1-16).

    Currently, tournament data is stored in this file as static data in
    private array TOURNAMENT_DATA_ARRAY:

    TOURAMENT_DATA_ARRAY =
    [
        ( year, &<TournamentData> ),
        ...
    ]

    where <&TournamentData> is ref to a static array of 64 tuples
    of ( team, region, seed ) info:

    TournamentData =
    [
        ( team, region, seed ),     // team 0
        ....
        ( team, region, seed ),     // team 63
    ]

    ** NOTE **
    The ordering of the tournament field teams is significant:
        - The teams in a region must be grouped together and ordered
          by seed 1-16.
        - The region-groups must be ordered so that the first region
          winner plays the second (in the final four) and the third
          plays the fourth; i.e. in 2023 (see below) the South plays
          the East and the Midwest plays the West in the final 4.

    TODO
    Put this stuff in a database
  _____________________________________________________________________________
*/
use std::collections::HashSet;

/**
 * TournamentData
 * A 64-team field
 * Static array of (team, region, seed)
 */
pub type TournamentData = [(&'static str, &'static str, u8); 64];

/**
 * get_tournament_data
 * Returns an Result<&TournamentData> of the field for the
 * specified year (Err = String error message).
 */
pub fn get_tournament_data(year: &str) -> Result<&TournamentData, String> {
    for t in TOURNAMENT_DATA_ARRAY {
        if t.0 == year {
            validate_tournament_data(t.0, &t.1)?;
            return Ok(&t.1);
        }
    }
    return Err(format!(
        "** ERROR **: tourney data not found for '{}'",
        year
    ));
}

/**
 * validate_tournament_data
 * Check validity of a tournament data (this is just to make sure I didn't mis-type
 * the "datastore" below):
 *     4 unique regions
 *     seeds ordered 1-16 within region
 *     regions must be contiguous in the list
 *     unique team names
 */
fn validate_tournament_data(year: &str, data: &TournamentData) -> Result<(), String> {
    let mut team_hash = HashSet::new();
    let mut region_hash = HashSet::new();
    let mut i: u32 = 1;
    let mut expected_seed_count: u8 = 1;
    let mut current_region: &str = "";
    for team_tuple in data.iter() {
        // no blank entries
        if team_tuple.0.len() == 0 || team_tuple.1.len() == 0 {
            return Err(format!(
                "** ERROR **: {} tournament data: entry {}: illegal empty team or region",
                year, i
            ));
        }
        // unique teams
        if !team_hash.insert(team_tuple.0) {
            return Err(format!(
                "** ERROR **: {} tournament data: entry {}: duplicate team: {}",
                year, i, team_tuple.0
            ));
        }
        // seed counts must be 1-16
        if team_tuple.2 != expected_seed_count {
            return Err(format!(
                "** ERROR **: {} tournament data: entry {}: unexpected seed: {}",
                year, i, team_tuple.2
            ));
        }
        // must have 4 regions
        if expected_seed_count == 1 {
            if i > 1 && team_tuple.1 == current_region {
                return Err(format!("** ERROR **: {} tournament data: entry {}: unexpected region (did not change): {}", year, i, team_tuple.1));
            }
            current_region = team_tuple.1;
            // unique regions
            if !region_hash.insert(current_region) {
                return Err(format!(
                    "** ERROR **: {} tournament data: entry {}: regions not unique: {}",
                    year, i, team_tuple.1
                ));
            }
        }
        // regions must be contiguous
        else if team_tuple.1 != current_region {
            return Err(format!(
                "** ERROR **: {} tournament data: entry {}: unexpected region {} (expected {})",
                year, i, team_tuple.1, current_region
            ));
        }
        expected_seed_count = if expected_seed_count == 16 {
            1
        } else {
            expected_seed_count + 1
        };
        i = i + 1;
    }
    return Ok(());
}

/**
 * TOURNAMENT_DATA_ARRAY
 * Static array of tournament data by year
 */
// #[allow(dead_code)]
type TournamentTuple = (&'static str, &'static TournamentData);
// #[allow(dead_code)]
static TOURNAMENT_DATA_ARRAY: [TournamentTuple; 1] = [("2023", &TOURNAMENT_DATA_2023)];

//__________________________________________________________
// 2023 NCAA men's tournament data
// #[allow(dead_code)]
static TOURNAMENT_DATA_2023: TournamentData = [
    ("Alabama", "South", 1),
    ("Arizona", "South", 2),
    ("Baylor", "South", 3),
    ("Virginia", "South", 4),
    ("San Diego St.", "South", 5),
    ("Creighton", "South", 6),
    ("Missouri", "South", 7),
    ("Maryland", "South", 8),
    ("West Virginia", "South", 9),
    ("Utah St.", "South", 10),
    ("NC State", "South", 11),
    ("Charleston", "South", 12),
    ("Furman", "South", 13),
    ("UCSB", "South", 14),
    ("Princeton", "South", 15),
    ("A&M CC", "South", 16),
    ("Purdue", "East", 1),
    ("Marquette", "East", 2),
    ("Kansas St.", "East", 3),
    ("Tennessee", "East", 4),
    ("Duke", "East", 5),
    ("Kentucky", "East", 6),
    ("Michigan St.", "East", 7),
    ("Memphis", "East", 8),
    ("Fla. Atlantic", "East", 9),
    ("USC", "East", 10),
    ("Providence", "East", 11),
    ("Oral Roberts", "East", 12),
    ("Louisiana", "East", 13),
    ("Montana State", "East", 14),
    ("Vermont", "East", 15),
    ("FDU", "East", 16),
    ("Houston", "Midwest", 1),
    ("Texas", "Midwest", 2),
    ("Xavier", "Midwest", 3),
    ("Indiana", "Midwest", 4),
    ("Miami", "Midwest", 5),
    ("Iowa St.", "Midwest", 6),
    ("Texas A&M", "Midwest", 7),
    ("Iowa", "Midwest", 8),
    ("Auburn", "Midwest", 9),
    ("Penn St.", "Midwest", 10),
    ("Pitt", "Midwest", 11),
    ("Drake", "Midwest", 12),
    ("Kent St.", "Midwest", 13),
    ("Kennesaw", "Midwest", 14),
    ("Colgate", "Midwest", 15),
    ("No Kentucky", "Midwest", 16),
    ("Kansas", "West", 1),
    ("UCLA", "West", 2),
    ("Gonzaga", "West", 3),
    ("UConn", "West", 4),
    ("St. Marys", "West", 5),
    ("TCU", "West", 6),
    ("Northwestern", "West", 7),
    ("Arkansas", "West", 8),
    ("Illinois", "West", 9),
    ("Boise St.", "West", 10),
    ("Nevada", "West", 11),
    ("VCU", "West", 12),
    ("Iona", "West", 13),
    ("Grand Canyon", "West", 14),
    ("UNC Ashville", "West", 15),
    ("Howard", "West", 16),
];
