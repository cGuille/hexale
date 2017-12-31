use uuid::Uuid;

#[derive(Debug)]
struct GameId(Uuid);

impl Default for GameId {
    fn default() -> GameId {
        GameId(Uuid::new_v4())
    }
}

#[derive(Debug, Default)]
pub struct Game {
    id: GameId,
    player1: Player,
    player2: Player,
    turn: Turn,
}

impl Game {
    pub fn id(&self) -> String {
        self.id.0.to_string()
    }

    pub fn set_turn(&mut self, new_turn: Turn) {
        self.turn = new_turn;
    }
}

#[derive(Debug, Default)]
struct Player {
    name: Option<String>,
    word_chosen: Option<String>,
    guess_count: u32,
}

#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2,
}

impl Default for Turn {
    fn default() -> Self {
        Turn::Player1
    }
}
