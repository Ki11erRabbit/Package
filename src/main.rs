mod pacman;
mod config_file;

use std::env;
use std::process::Command;



/*
 *  MAIN
 *
 *  takes in command line arguments and puts them in an array.
 *  input variable is a struct Data that contains the command, options,
 *  and remaining arguments for Package.
*/

fn main () {
    config_file::check_config();

    let args: Vec<String> = env::args().collect();

    let input = parse_arguments(args);



    if input.option.as_str() == "ERROR" {
        println!("Invalid command -- \'{}\'", input.command);
        //TODO: call function that executes the various package managers possible
    }
    else {
        println! ("command: {} \noption: {}", input.command, input.option);
        //execute_package_manager(String::from("pacman"), String::from("-S"), [String::from("telegram-desktop"), String::from("shotwell")]);
    }

}

fn execute_command (input: Data) {
    let selected_pkg_mgr: String;
    if input.command.eq("initialize") {
        config_file::reinitialize_config();
    }
    else if match_option(&input.option) {
        if config_file::check_available(String::from(input.args[0].as_str())) {
            println!("TODO: add in ability to launch into particular package manager.");
            selected_pkg_mgr = String::from(input.args[0].as_str());
        }
    }
    else {
        selected_pkg_mgr = config_file::get_current();
    }


}

fn prehand_off (input: Data, current_pkg_mgr: String) {
    if current_pkg_mgr.eq("pacman") {
        pacman::hand_off(input.command, input.args);
    }
    else if current_pkg_mgr.eq("apt") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("dnf") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("portage") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("zypper") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("snap") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("flatpak") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("aur") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("npm") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else if current_pkg_mgr.eq("pip") {
        println!("Package Manager {} not yet implemented.", current_pkg_mgr);
    }
    else {
        println!("Invalid package manager {}", current_pkg_mgr)
    }
}


fn prepare (input: Data) {
    //TODO: write a function that reads a configuration file and selects default package manager which then feeds the input to the various package manager .rs files

    //if input.command.eq("update")

}

/* TODO: move to individual package manager .rs file
    in order for args() to work, the array size must be known at compile time. The likely fix will be to have an array that is size 50 at least or use vectors

*/
fn execute_package_manager (program_command: &String, program_args: &String, remaining_args: &Vec<String>) {

    let mut child = Command::new("program_command")
            .arg("program_args")
            .args(remaining_args)
            .spawn().unwrap();

    let result = child.wait().unwrap();

    /*Command::new(program_command)
        .arg(program_args)
        .arg(remaing_args)
        .spawn();*/
}



/*
 *  Input Interpretation
*/

struct Data {
    command: String,//the task for package to perform
    option: String, //any options in "-xyz" format
    args: Vec<String>,   //all remaining arguments //TODO change to vector
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
    let mut arguments = Vec::new();

    if match_command(&command) {

        for i in 2..args.len()  {
            if match_option(&args[i]) {
                option.push_str(args[i].as_str());
            }
            else {
                arguments.push(String::from(args[i].as_str()));
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
            args: vec![String::from("ERROR")],
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
