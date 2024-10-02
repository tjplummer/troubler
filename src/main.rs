use std::ops::{Index, IndexMut};
use rand::Rng;
use rand::rngs::ThreadRng;
use argh::FromArgs;
use rand::distributions::{Distribution, Standard};

#[derive(PartialEq, Clone, Copy)]
enum Players {
    Red,
    Green,
    Yellow,
    Blue
}

impl Distribution<Players> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Players {
        match rng.gen_range(0..=3) {
            0 => Players::Red,
            1 => Players::Green,
            2 => Players::Yellow,
            _ => Players::Blue
        }
    }
}

impl Players {
    fn get_players(num: u8) -> Vec<Players> {
        let mut players: Vec<Players> = Vec::new();

        for _ in 0..num {
            loop {
                let player : Players = rand::random();

                if (players.contains(&player)) { continue; } else {
                    players.push(player);
                    break;
                }
            }
        }

        players
    }
}

struct Player {
    name: Players,
    pieces_not_in_game: Vec<Piece>,
    pieces_in_game: Vec<Piece>,
    pieces_complete: Vec<Piece>
}

impl Player {
    fn new(name: Players) -> Player {
        let mut pieces: Vec<Piece> = Vec::new();

        for _ in 0..3 {
            pieces.push(Piece::new(String::from("None"), name))
        }

        Player {
            name,
            pieces_not_in_game: pieces,
            pieces_in_game: Vec::new(),
            pieces_complete: Vec::new(),
        }
    }

    fn new_game_players(names: Vec<Players>) -> Vec<Player> {
        let mut players: Vec<Player> = Vec::new();

        for name in names {
            players.push(Player::new(name));
        }

        players
    }

    fn check_if_pieces_in_play(&self) -> bool {
        if self.pieces_in_game.is_empty() { false } else { true }
    }

    fn move_piece_into_game(&mut self) {

    }
}


struct Piece {
    location: String,
    owner: Players
}

impl Piece {
    fn new(location: String, owner: Players) -> Piece {
        Piece {
            location,
            owner
        }
    }
}


struct Board {
    r1: Option<Piece>,
    r2: Option<Piece>,
    r3: Option<Piece>,
    r4: Option<Piece>,
    r5: Option<Piece>,
    r6: Option<Piece>,
    g1: Option<Piece>,
    g2: Option<Piece>,
    g3: Option<Piece>,
    g4: Option<Piece>,
    g5: Option<Piece>,
    g6: Option<Piece>,
    y1: Option<Piece>,
    y2: Option<Piece>,
    y3: Option<Piece>,
    y4: Option<Piece>,
    y5: Option<Piece>,
    y6: Option<Piece>,
    b1: Option<Piece>,
    b2: Option<Piece>,
    b3: Option<Piece>,
    b4: Option<Piece>,
    b5: Option<Piece>,
    b6: Option<Piece>,
    rg: Option<Piece>,
    gy: Option<Piece>,
    yb: Option<Piece>,
    br: Option<Piece>
}

impl Default for Board {
    fn default() -> Board {
        Board {
            r1: None,
            r2: None,
            r3: None,
            r4: None,
            r5: None,
            r6: None,
            rg: None,
            g1: None,
            g2: None,
            g3: None,
            g4: None,
            g5: None,
            g6: None,
            gy: None,
            y1: None,
            y2: None,
            y3: None,
            y4: None,
            y5: None,
            y6: None,
            yb: None,
            b1: None,
            b2: None,
            b3: None,
            b4: None,
            b5: None,
            b6: None,
            br: None,
        }
    }
}

impl Index<&'_ str> for Board {
    type Output = Option<Piece>;
    fn index(&self, s: &str) -> &Option<Piece> {
        match s {
            "r1" => &self.r1,
            "r2" => &self.r2,
            /// Do all these
            _ => panic!("unknown field: {}", s),
        }
    }
}

/// I don't think I need both, but maybe?
impl IndexMut<&'_ str> for Board {
    fn index_mut(&mut self, s: &str) -> &mut Option<Piece> {
        match s {
            "r1" => &mut self.r1,
            "r2" => &mut self.r2,
            /// do rest
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Board {
    fn update_board(&mut self, player: Players, roll: u8) {
        /// needs to return a bool to let know a trouble occurs
    }
}

struct GameState {
    board: Board,
    players: Vec<Player>,
    turn: i32,
}

impl GameState {
    fn new(board: Board, players: Vec<Player>) -> GameState {
        GameState {
            board,
            players,
            turn: 0
        }
    }

    fn roll_dice(mut rng: ThreadRng) -> u8 {
        rng.gen_range(1..6)
    }

    fn can_exit(roll: u8) -> bool {
        if roll == 1 | 6 { true } else { false }
    }

    fn can_roll_again(roll: u8) -> bool {
        if roll == 6 { true } else { false }
    }
}

fn default_players() -> u8 {
    3
}

fn default_quantity() -> u32 {
    1
}

#[derive(FromArgs)]
/// Exercise in Trouble
struct Args {
    /// an optional parameter for players. Default is 4
    #[argh(option, short = 'p', default = "default_players()")]
    players: u8,

    /// an optional parameter for games to play. Default is 1
    #[argh(option, short = 'q', default = "default_quantity()")]
    quantity: u32
}

fn main() {
    let args: Args = argh::from_env();

    let mut game_state = GameState::new(
        Board::default(),
        Player::new_game_players(
            Players::get_players(args.players)
        )
    );

    /// Start game
    ///
    /// Finish game
    ///
    /// Logging
    ///
    /// Database

    println!("Hello, world!");
}
