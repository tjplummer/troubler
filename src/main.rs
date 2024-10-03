use std::ops::{Index, IndexMut};
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use argh::FromArgs;
use rand::distributions::{Distribution, Standard};


#[derive(PartialEq, Clone, Copy)]
enum AvoidanceStrategy {
    Scared = 6,
    Cautious = 4,
    ImmediateOnly = 2,
    Indifferent = 0
}

impl Distribution<AvoidanceStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AvoidanceStrategy {
        match rng.gen_range(0..=3) {
            0 => AvoidanceStrategy::Cautious,
            1 => AvoidanceStrategy::ImmediateOnly,
            2 => AvoidanceStrategy::Indifferent,
            _ => AvoidanceStrategy::Scared
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum MovementStrategy {
    Leader,
    Follower,
}

impl Distribution<MovementStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MovementStrategy {
        match rng.gen_range(0..=1) {
            0 => MovementStrategy::Leader,
            _ => MovementStrategy::Follower,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum AggressionStrategy {
    Balanced = 50,
    Lean = 60,
    Heavy = 70,
    Extreme = 80
}

impl Distribution<AggressionStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AggressionStrategy {
        match rng.gen_range(0..=3) {
            0 => AggressionStrategy::Balanced,
            1 => AggressionStrategy::Lean,
            2 => AggressionStrategy::Heavy,
            _ => AggressionStrategy::Extreme
        }
    }
}


#[derive(PartialEq, Clone, Copy)]
enum PieceStrategy {
    GetOnBoard,
    FinishPiece
}

impl Distribution<PieceStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceStrategy {
        match rng.gen_range(0..=1) {
            0 => PieceStrategy::FinishPiece,
            _ => PieceStrategy::GetOnBoard,
        }
    }
}

struct AI {
    piece_strategy: PieceStrategy,
    aggression_strategy: AggressionStrategy,
    movement_strategy: MovementStrategy,
    avoidance_strategy: AvoidanceStrategy
}

impl AI {
    fn new_random() -> Self {
        let piece_strategy : PieceStrategy = rand::random();
        let aggression_strategy : AggressionStrategy = rand::random();
        let movement_strategy : MovementStrategy = rand::random();
        let avoidance_strategy : AvoidanceStrategy = rand::random();

        AI {
            piece_strategy,
            aggression_strategy,
            movement_strategy,
            avoidance_strategy
        }
    }
}


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

    fn check_if_pieces_available_to_play(&self) -> bool {
        if self.pieces_not_in_game.is_empty() { false } else { true }
    }

    fn move_piece_into_game(&mut self) {
        self.pieces_in_game.push(self.pieces_not_in_game.pop().unwrap());
    }
}


struct Piece {
    start_index: u8,
    location: String,
    owner: Players
}

impl Piece {
    fn new(location: String, owner: Players) -> Piece {
        Piece {
            start_index: 0,
            location,
            owner
        }
    }

    fn add_to_board(&mut self) {
        match self.owner {
            Players::Red => self.start_index = 2,
            Players::Green => self.start_index = 9,
            Players::Yellow => self.start_index = 16,
            Players::Blue => self.start_index = 23,
        }
    }

    fn remove_from_board(&mut self) {
        self.start_index = 0;
        self.location = String::new()
    }

    fn update_location(&mut self, location: String) {
        self.location = location;
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
            "r3" => &self.r3,
            "r4" => &self.r4,
            "r5" => &self.r5,
            "r6" => &self.r6,
            "rg" => &self.rg,
            "g1" => &self.g1,
            "g2" => &self.g2,
            "g3" => &self.g3,
            "g4" => &self.g4,
            "g5" => &self.g5,
            "g6" => &self.g6,
            "gy" => &self.gy,
            "y1" => &self.y1,
            "y2" => &self.y2,
            "y3" => &self.y3,
            "y4" => &self.y4,
            "y5" => &self.y5,
            "y6" => &self.y6,
            "yb" => &self.yb,
            "b1" => &self.b1,
            "b2" => &self.b2,
            "b3" => &self.b3,
            "b4" => &self.b4,
            "b5" => &self.b5,
            "b6" => &self.b6,
            "br" => &self.br,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ u8> for Board {
    fn index_mut(&mut self, s: &u8) -> &mut Option<Piece> {
        match s {
            0 => &mut self.r1,
            1 => &mut self.r2,
            2 => &mut self.r3,
            3 => &mut self.r4,
            4 => &mut self.r5,
            5 => &mut self.r6,
            6 => &mut self.rg,
            7 => &mut self.g1,
            8 => &mut self.g2,
            9 => &mut self.g3,
            10 => &mut self.g4,
            11 => &mut self.g5,
            12 => &mut self.g6,
            13 => &mut self.gy,
            14 => &mut self.y1,
            15 => &mut self.y2,
            16 => &mut self.y3,
            17 => &mut self.y4,
            18 => &mut self.y5,
            19 => &mut self.y6,
            20 => &mut self.yb,
            21 => &mut self.b1,
            22 => &mut self.b2,
            23 => &mut self.b3,
            24 => &mut self.b4,
            25 => &mut self.b5,
            26 => &mut self.b6,
            27 => &mut self.br,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Board {
    fn update_board(&mut self, player: Players, roll: u8) -> bool {
        /// TODO: All logic
        false
    }

    fn new_board_position(&mut self, player: Player, roll: u8) -> String {
        /// TODO: All logic
        String::new()
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

    fn player_turn(mut rng: ThreadRng, player: Player) {
        let roll = Self::roll_dice(rng);

        /// TODO: Logic on what to do

        /// TODO: Check if pieces in play
        /// TODO: Check if can exit
        /// TODO: Exit or End
        /// TODO: Can roll again
    }
}

fn default_players() -> u8 { 3 }
fn default_quantity() -> u32 { 1 }

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
    let rng: ThreadRng = thread_rng();

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
