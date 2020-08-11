use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Commands {
    Go,
    Take,
    Attack,
    Quit,
    Unknown,
}

#[derive(PartialEq)]
pub enum Actions {
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

pub struct GameTile {
    pub display_message: String,
    pub enemy: Option<Enemy>,
    pub item: String,
    pub actions: Vec<Actions>,
}

pub struct Command {
    pub command: String,
    pub action: String,
    pub modifier: String,
}

impl Command {
    pub fn new() -> Command {
        let new_command = Command {
            command: String::from(""),
            action: String::from(""),
            modifier: String::from(""),
        };

        new_command
    }

    fn get_command(&self) -> Commands {
        match self.command.as_str() {
            "go" => return Commands::Go,
            "move" => return Commands::Go,
            "attack" => return Commands::Attack,
            "take" => return Commands::Take,
            "quit" => return Commands::Quit,
            _ => return Commands::Unknown,
        };
    }

    fn get_action(&self) -> Actions {
        match self.action.as_str() {
            "right" => return Actions::Right,
            "left" => return Actions::Left,
            "up" => return Actions::Up,
            "down" => return Actions::Down,
            _ => return Actions::Unknown,
        }
    }
}

pub struct Player {
    pub level: i32,
    pub location: i32,
    pub health: i32,
    pub inventory: Vec<String>,
}

pub struct Enemy {
    pub name: String,
    pub starting_health: i32,
    pub current_health: i32,
}

impl Enemy {
    // TODO
}

pub struct GameState {
    pub player: Player,
    world: HashMap<i32, HashMap<i32, GameTile>>,
    pub has_won: bool,
    pub game_end: bool,
}

impl GameState {
    pub fn new() -> GameState {
        let game = GameState {
            player: Player {
                level: 0,
                location: 0,
                health: 100,
                inventory: Vec::new(),
            },
            has_won: false,
            game_end: false,
            world: build_world(),
        };

        game
    }

    pub fn get_location(&self) -> &GameTile {
        let player_level = self.world.get(&self.player.level)
            .expect("Unknown player level.");
        let player_location = player_level.get(&self.player.location)
            .expect("Unknown player location.");

        player_location
    }

    pub fn take_action(&mut self, command: Command) -> Option<String> {
        println!("command: {} action: {}", command.command, command.action);

        let converted_command = command.get_command();
        let converted_action = command.get_action();

        match converted_command {
            Commands::Go => {
                if self.get_location().actions.contains(&converted_action) {
                    match converted_action {
                        Actions::Left => self.player.location -= 1,
                        Actions::Right => self.player.location += 1,
                        _ => return Some(String::from("Matt's lazy and hasn't programmed any other actions yet.")),
                    };
                } else {
                    return Some(String::from("nope, you can't do that."));
                }
            },
            Commands::Attack => {
                match &self.get_location().enemy {
                    Some(e) => {
                        // todo
                    },
                    None => return Some(String::from("There is no enemy here to fight.")),
                }
            },
            Commands::Quit => {
                self.game_end = true;
            }
            _ => {
                return Some(String::from("Command was not recognized or programmed yet."));
            },
        }

        None
    }
}

fn build_world() -> HashMap<i32, HashMap<i32, GameTile>> {
    let mut world = HashMap::new();
    let mut level = HashMap::new();

    let zero = GameTile {
        display_message: String::from("You are standing outside under a bright sunny sun.  To the right there is a white house with no windows."),
        actions: vec![Actions::Right],
        enemy: None,
        item: String::from("")
    };

    let one = GameTile {
        display_message: String::from("The sun is still shiny, no birds in the air.  There's no way into the house There is a sword stuck in a stump, a path leading left and a path leading right."),
        actions: vec![Actions::Right, Actions::Left],
        enemy: None,
        item: String::from("sword")
    };

    let two = GameTile {
        display_message: String::from("As you enter the clearing, the world seems to get darker as the foreboding trees block out the sun.  To the left is a white house, and to the right is a dark cave."),
        actions: vec![Actions::Right, Actions::Left],
        enemy: None,
        item: String::from("")
    };

    let three = GameTile {
        display_message: String::from("You are standing outside under a bright sunny sun.  To the right there is a white house with no windows."),
        actions: vec![Actions::Left],
        enemy: Some(Enemy {
            name: String::from("Grue"),
            starting_health: 5,
            current_health: 5,
        }),
        item: String::from("")
    };

    level.insert(0, zero);
    level.insert(1, one);
    level.insert(2, two);
    level.insert(3, three);

    world.insert(0, level);

    world
}
