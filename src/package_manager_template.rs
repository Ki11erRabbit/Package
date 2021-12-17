use std::env;
use std::process::Command;
use std::process::Child;


pub fn hand_off (command: String, option: String, args: Vec<String>) {



}

fn install (args: Vec<String>) {
    let mut child = Command::new()
            .arg()
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn remove (args: Vec<String>) {
    let mut child = Command::new()
            .arg()
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn upgrade (args: Vec<String>) {
    let mut child = Command::new()
            .arg()
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn search (args: Vec<String>) {
    let mut child = Command::new()
            .arg()
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

fn pass (option: String, args: Vec<String>) {
    let mut child = Command::new()
            .arg(option)
            .args(args)
            .spawn().unwrap();

    let result = child.wait().unwrap();
}

