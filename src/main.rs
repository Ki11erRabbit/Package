use std::io;

enum Commands {
    Install,
    Remove,
    Update,
    Search,
    Set,
    Pass,
    Initialize,
    Help,
}

fn main() {
    let mut command = String::new();
    let mut option = String::new();
    let mut args = String::new();
    let args: Vec<String> = env::args().collect();

    //TODO: change to use command line arguments
    io::stdin()
        .read_line(&mut input);
}

fn parse_input (input: String, command: &mut String, option: &mut String, args: &mut String) {
    let vect: Vec<&str> = input.split(' ').collect();
    let mut foundCommand = false;

    for element in vect {
        match_command(command, vect[1]);
    }
}

fn match_command (command: &mut String, input: &str) -> bool {

    if input.eq("install") {
        return true
    }
    /*match (*input) {
        install => {
            *command = String::from("install");
            true
        }
        "remove" => {
            *command = String::from("install");
            true
        }
    }*/
    false
}
