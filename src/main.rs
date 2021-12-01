use std::io;

enum Commands {
    Install,
    Remove,
    Update,
    Search,
    Set,
    Pass:,
    Initialize,
    Help,
}

fn main() {
    let mut command = String::new();
    let mut option = String::new();
    let mut args = String::new();
    let mut input = String::new();

    //TODO: change to use command line arguments
    io::stdin()
        .read_line(&mut input);
}

fn parse_input (input: String, command: &String, option: &String, args: &String) {
    let vect: Vec<&str> = input.split(' ').collect();
    let mut foundCommand = false;

    for element in vect {

    }
}

fn match_command (command: &String, input: String) -> bool {
    match command {
        Commands::Install.to_string() => {
            command = Commands::Install.to_string();
            true
        }
        Commands::Remove.to_string() => {
            command = Commands::Remove.to_string();
        }
    }

}
