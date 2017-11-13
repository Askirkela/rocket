use super::Json;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Move {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    chessman: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Game {
    id: String,
    moves: Vec<Move>
}
impl Game {
    pub fn new(ident: String) -> Game {
        Game {
            id: ident,
            moves: Vec::new()
        }
    }
    pub fn add_move(&mut self, m: Move) {
        self.moves.push(m);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    rank: u64,
    points: u64,
    name: String
}
impl User {
    pub fn new(n: &String, r: u64, p: u64) -> User {
        User {
            name: n.to_ascii_lowercase(),
            rank: r,
            points: p
        }
    }
}

pub fn get_user(name: &String) -> Json<User> {
    Json(User::new(&name, 1, 1200))
}

pub fn game(gameid: String) -> Json<Game> {
    Json(Game::new(gameid))
}