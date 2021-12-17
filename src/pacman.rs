use std::env;
use std::process::Command;
use std::process::Child;

/*struct Data {
    command: String,//the task for package to perform
    option: String, //any options in "-xyz" format
    args: Vec<String>,   //all remaining arguments
}*/

pub fn hand_off (command: String, option: String, args: Vec<String>) {



}

fn install (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .arg("-S")
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn remove (args: Vec<String>) {

}

fn upgrade (args: Vec<String>) {

}

fn search (args: Vec<String>) {

}

fn pass (args: Vec<String>) {

}

