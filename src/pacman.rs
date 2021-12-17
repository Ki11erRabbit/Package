use std::env;
use std::process::Command;
use std::process::Child;

struct Data {
    command: String,//the task for package to perform
    option: String, //any options in "-xyz" format
    args: vec<String>,   //all remaining arguments
}

pub fn hand_off (input: Data) {

}

fn install () {
    let mut child = Command::new("pacman")
            .arg("-S")
            .args(["telegram-desktop", "shotwell"])
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn remove () {

}

fn upgrade () {

}

fn search () {

}

fn pass () {

}

