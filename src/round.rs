/*_____________________________________________________________________________
    Round

    All the games played in a specific round of the tournament:

        Round 0: 64 teams playing 32 games (the first round)
        Round 1: 32 teams playing 16 games (the round of 32)
        Round 2: 16 teams playing 8 games (the sweet sixteen)
        ...
        Round 5: 2 teams playing 1 game (the final)

    The Round struct uses a Vec to store its games (so that a single
    Round struct can accomodate any of the rounds).

    NOTE: the order of the games within the round is significant because
    it encodes the tournament structure; i.e. the winner of game 0 of the
    round (first Vec entry) will play the winner of game 1 of the round
    (second Vec entry) in the next round.
  _____________________________________________________________________________
*/
use crate::game::Game;
use std::vec;

/**
 * A round of the tournament
 * Round N (0..5) contains (2^(5-N)) ordered games
 * Round name is for descriptive name: e.g. "Sweet Sixteen"
 */
#[derive(Debug, Default)]
pub struct Round<'r> {
    pub name: String,
    pub num: u8,
    pub games: Option<Vec<Game<'r>>>,
}

/**
 * Round constructor
 * The constructor creates the round's games and initializes them (e.g.
 * scores = 0, etc) without assigning teams (these will be filled in
 * when a user makes picks) or initializing the game's "previous"
 * and "next" game pointers (this is done by the Bracket struct,
 * which holds all the tournament rounds).
 */
impl<'r> Round<'r> {
    pub fn new(name: String, num: u8) -> Self {
        assert!(num <= 5);
        let base: usize = 2;
        // make Vec have the correct number of games for this round
        let p: u32 = u32::from(5 - num);
        let mut game_vec: Vec<Game<'r>> = vec![Game::<'r>::default(); base.pow(p)];
        // initialize all the games in the round
        for g in game_vec.iter_mut() {
            // Note: the Vec assignment above probably does exactly this (except
            // for putting the correct round, but I like being explicit).
            (*g).round = num;
            (*g).score_home = 0;
            (*g).score_away = 0;
            (*g).completed = false;
            (*g).team_home = None;
            (*g).team_away = None;
            (*g).prev = [None; 2];
            (*g).next = None;
        }
        Self {
            name,
            num,
            games: Some(game_vec),
        }
    }
}
