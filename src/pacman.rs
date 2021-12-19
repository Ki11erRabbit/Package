use std::process::Command;

pub fn hand_off (command: String, args: Vec<String>) {

    if command.eq("install") {
        install(args);
    }
    else if command.eq("remove") {
        remove(args);
    }
    else if command.eq("update") {
        upgrade(args);
    }
    else if command.eq("search") {
        search(args);
    }
    else if command.eq("pass") {
        pass(args);
    }
    else {
        println!("Invalid Command");
    }

}

fn install (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .arg("-S")
            .args(args)
            .spawn().unwrap();

    let _result = child.wait().unwrap();
}

fn remove (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .arg("-R")
            .args(args)
            .spawn().unwrap();

    let _result = child.wait().unwrap();
}

fn upgrade (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .arg("-Syu")
            .args(args)
            .spawn().unwrap();

    let _result = child.wait().unwrap();
}

fn search (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .arg("-Ss")
            .args(args)
            .spawn().unwrap();

    let _result = child.wait().unwrap();
}

fn pass (args: Vec<String>) {
    let mut child = Command::new("pacman")
            .args(args)
            .spawn().unwrap();

    let _result = child.wait().unwrap();
}
