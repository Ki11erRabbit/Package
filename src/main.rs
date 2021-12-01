use std::io;
use std::env;

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

struct Data {
    command: String,
    option: String,
    args: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = parse_arguments(args);
    println! ("command: {} \noption: {} \narguments: {}", input.command, input.option, input.args);
}

fn parse_arguments (args: Vec<String>) -> Data {
    let command: &String = &args[1];
    let mut option: String = String::from("");
    let mut arguments: String = String::from("");
    if match_command(&command) {

        for i in 2..args.len()  {
            if match_option(&args[i]) {
                option.push_str(args[i].as_str());
            }
            else {
                arguments.push_str(args[i].as_str());
                arguments.push_str(" ");
            }
        }
        //println!("{}", arguments);
        return Data {
            command: command.to_string(),
            option: option.to_string(),
            args: arguments,
        }

    }
    else {
        println!("Command not found");
    }
    Data {
        command: String::from("ERROR"),
        option: String::from("ERROR"),
        args: String::from("ERROR"),
    }
}

fn match_command (input: &String) -> bool {

    if input.eq("install") {
        //println!("install");
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

fn match_option (input: &String) -> bool {
    if input.eq("-use") {
        //println!("-use");
        return true
    }
    else {
        return false
    }
}




