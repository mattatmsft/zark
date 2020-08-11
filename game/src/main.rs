mod zark;
use std::io::{self, Read, Write};

fn main() {
    print_output(String::from("Welcome to Zark, the world exploring game from MW."));
    println!("");
    let commands = vec!["go", "move", "take", "attack", "quit"];
    print_output(String::from("Available commands: "));
    println!("{:?}", commands);
    println!("");

    // set up game
    let mut game = zark::GameState::new();

    while game.game_end != true {
        // game loop
        // write location
        print_output(game.get_location().display_message.to_string());
        let mut player_input = String::new();

        // ask for instruction
        print_output(String::from("What would you like to do?"));
        match io::stdin().read_line(&mut player_input) {
            Ok(buffer) => println!("buffer read: {}", buffer),
            Err(error) => println!("Error: {}", error),
        };

        println!("input: {}", player_input.trim());

        // take action
        // extra game over conditions here.  Like game won.  Or stuff
        let command = generate_command(&player_input);
        game.take_action(command);

        print_state(&game);
        println!("");
    }

    if game.has_won {
        println!("Congratulations, you lived!!");
    } else { 
        println!("Boo, you lost.");
    }
}

fn generate_command(user_input: &String) -> zark::Command {
    let mut player_command = zark::Command::new();

    let mut count = 0;

    for input in user_input.split_whitespace() {
        match count {
            0 => player_command.command = input.to_string(),
            1 => player_command.action = input.to_string(),
            2 => player_command.modifier = input.to_string(),
            _ => println!("Woops too many words"),
        }

        count += 1;
    }
    player_command
}

fn print_output(message: String) {
    println!("{}", message);
    io::stdout()
        .flush()
        .unwrap(); // unsafe, but I'm ok with it here.
}

fn print_state(game_state: &zark::GameState) {
    println!("");
    println!("state------------------");
    println!("Game end: {}", game_state.game_end);
    println!("Health: {}", game_state.player.health);
    println!("Won: {}", game_state.has_won);
    println!("/state-----------------");
    println!("");
}
