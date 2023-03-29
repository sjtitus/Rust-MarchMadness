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
#[derive(Debug, Default, Clone)]
pub struct Round {
    pub name: String,
    pub num: u8,
    pub games: Vec<Game>
}

/**
 * Round constructor
 * The constructor creates the round's games and initializes them (e.g.
 * scores = 0, etc) without assigning teams (these will be filled in
 * when a user makes picks).
 */
impl Round {
    pub fn new(name: String, num: u8) -> Self {
        assert!(num <= 5);
        let base: usize = 2;
        // make Vec have the correct number of games for this round
        let p: u32 = u32::from(5 - num);
        let mut game_vec: Vec<Game> = vec![Game::default(); base.pow(p)];
        // initialize all the games in the round
        for (i, g) in game_vec.iter_mut().enumerate() {
            // Note: the Vec assignment above probably does exactly this (except
            // for putting the correct round, but I like being explicit).
            (*g).teams = [None, None];
            (*g).score = [0, 0];
            (*g).completed = false;
            (*g).round = num;
            (*g).index = i;
        }
        Self {
            name,
            num,
            games: game_vec,
        }
    }
}
