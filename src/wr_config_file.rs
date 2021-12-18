use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Deserialize, Debug)]
struct Config {
    current: String,
    pacman: bool,
    apt: bool,
    dnf: bool,
    zypper: bool,
    portage: bool,
    flatpak: bool,
    aur: bool,
    snap: bool,
    npm: bool,
    pip: bool,
}

pub fn check_config () {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {

        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("config.toml"),);

        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => create_config(),
        };

        dbg!(config);

        dbg!(config_dir);
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }


}

fn create_config() -> Config {//TODO: Make it automatic rather than requiring user input
    println!("TODO: allow user to create config file");
    println!("What is the primary package manager that you use?");
    let mut main_pkgmgr = String::new();
    io::stdin()
            .read_line(&mut main_pkgmgr)
            .expect("Failed to read line");

    let mut secondary_pkgmgr = Vec::new();
    match yes_no_prompt() {
        false => println!("Writing Config File"),
        true => loop {
                println!("What is another package manager that you use?\n (i.e. pacman, apt, portage, npm)");
                let mut pkgmgr = String::new();
                io::stdin()
                    .read_line(&mut pkgmgr)
                    .expect("Failed to read line");
                secondary_pkgmgr.push(String::from(pkgmgr.as_str()));
                match yes_no_prompt() {
                    false => {
                        println!("Writing Config File");
                        break;
                        },
                    true => continue,
                };
            },
    };



    /*let yn_prompt: char = match guess.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };*/

    Config {
        current: String::from("ERROR"),
        pacman: false,
        apt: false,
        dnf: false,
        zypper: false,
        portage: false,
        flatpak: false,
        aur: false,
        snap: false,
        npm: false,
        pip: false,
    }
}

fn yes_no_prompt () -> bool {//TODO: come up with a better name
    println!("OK! Do you have any other package mangagers you wish to use? y/n");

    let mut yn_prompt = String::new();
    io::stdin()
            .read_line(&mut yn_prompt)
            .expect("Failed to read line");

    let yn_prompt: char = match yn_prompt.trim().parse() {
            Ok(char) => char,
            Err(_) => 'n',
        };

    if yn_prompt.eq(&'y') {
        true
    }
    else if yn_prompt.eq(&'n') {
        false
    }
    else {
        println!("Invalid response");
        false
    }

}

pub fn read_config () {

    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {
        let config_dir = proj_dirs.config_dir();

        dbg!(config_dir);
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }
}
