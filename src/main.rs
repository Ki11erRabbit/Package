use std::env;


/*
 *  MAIN
 *
 *  takes in command line arguments and puts them in an array.
 *  input variable is a struct Data that contains the command, options,
 *  and remaining arguments for Package.
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = parse_arguments(args);

    if input.option.as_str() == "ERROR" {
        println!("Invalid command -- \'{}\'", input.command);
    }
    else {
        println! ("command: {} \noption: {} \narguments: {}", input.command, input.option, input.args);
    }

}





/*
 *  Input Interpretation
*/

struct Data {
    command: String,//the task for package to perform
    option: String, //any options in "-xyz" format
    args: String,   //all remaining arguments
}

/*
 *  fn parse_arguments (args: Vec<String>) -> Data
 *
 *  takes a vector of Strings to process into a struct Data
 *  match_command checks for a valid command into package from the args vector
 *  if invalid, the values in Data will be the command then "ERROR" for everything else
 *  otherwise iterate through args to set option (which is checked by match_command)
 *  and then all remaining values in args is put in arguments. The variables command,
 *  option, and arguments are put into Data and returned.
*/

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
        Data {
            command: command.to_string(),
            option: option.to_string(),
            args: arguments,
        }

    }
    else {
        Data {
            command: command.to_string(),
            option: String::from("ERROR"),
            args: String::from("ERROR"),
        }
    }
}

/*
 *  fn match_command (input: &String) -> bool
 *
 *  takes in a &String and checks it against the possible commands to Package
 *  If a match is found then the function returns true. Otherwise returns false.
 *  A false means that the command was invalid and will terminate with an error.
*/

fn match_command (input: &String) -> bool {

    if input.eq("install") {
        true
    }
    else if input.eq("remove") {
        true
    }
    else if input.eq("update") {
        true
    }
    else if input.eq("search") {
        true
    }
    else if input.eq("set") {
        true
    }
    else if input.eq("pass") {
        true
    }
    else if input.eq("initialize") {
        true
    }
    else {
        false
    }
}

/*
 *  fn match_option (input: &String) -> bool
 *
 *  takes in a &String and checks it against the possible options for Package
 *  If a match is found then the function returns true. Otherwise returns false.
 *  A false means that there was no option found and does not terminate the program.
*/

fn match_option (input: &String) -> bool {
    if input.eq("-use") {
        true
    }
    else {
        false
    }
}
