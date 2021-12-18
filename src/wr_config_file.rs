use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

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

pub fn read_config () {

    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {
        let config_dir = proj_dirs.config_dir();

        dbg!(config_dir);
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }
}
