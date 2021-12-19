use directories::ProjectDirs;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use io::Write;

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    current: String,
    pacman: bool,
    apt: bool,
    dnf: bool,
    portage: bool,
    zypper: bool,
    snap: bool,
    flatpak: bool,
    aur: bool,
    npm: bool,
    pip: bool,
}
//TODO: add function that can change the current/default package manager
pub fn get_current () -> String {
    let mut output = String::new();

    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {

        let config_dir = proj_dirs.config_dir();
        let config_file_location = config_dir.join("config.toml");

        let config_file = fs::read_to_string(config_dir.join("config.toml"),);

        let config: Config = toml::from_str(&config_file.unwrap()).unwrap();

        output = config.current;
        //dbg!(config.current);
    }
    output
}

pub fn check_available (pkg_mgr: String) -> bool {
    let mut output: bool = false;

    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {

        let config_dir = proj_dirs.config_dir();
        let config_file_location = config_dir.join("config.toml");

        let config_file = fs::read_to_string(config_dir.join("config.toml"),);

        let config: Config = toml::from_str(&config_file.unwrap()).unwrap();

        if config.pacman || config.apt || config.dnf || config.portage || config.zypper ||
           config.zypper || config.flatpak || config.aur || config.npm {
            output = true;
        }
        else {
            output = false;
        }
    }
    output
}

pub fn reinitialize_config () {
    println!("Recreating configuration file");

    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {

        let config_dir = proj_dirs.config_dir();
        let config_file_location = config_dir.join("config.toml");

        let config_file = fs::read_to_string(config_dir.join("config.toml"),);

        //TODO: add in capability to delete config file for recreation
        let mut file = File::open(config_file_location)
            .expect("Could not open file!");
        let user_config: Config = create_config();//TODO: make it so that the file is cleared first
        let toml = toml::to_string(&user_config).unwrap();

        file.write_all(&toml.into_bytes())
            .expect("Cannot write to the file :(");

        dbg!(config_dir);
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }

}

pub fn check_config () {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "Ki11erRabbit",  "package") {

        let config_dir = proj_dirs.config_dir();
        let config_file_location = config_dir.join("config.toml");

        let config_file = fs::read_to_string(config_dir.join("config.toml"),);


        match config_file {//TODO: make fix this so that if the user ctr + c during setup it doesn't ever rewrite config file.
            Ok(file) => {},
            Err(_) => {
                    println!("Unable to find configuration file. Begining config file creation.");
                    fs::create_dir_all(config_dir);
                    let mut file = File::create(config_file_location)
                        .expect("Could not create file!");
                    let user_config: Config = create_config();
                    let toml = toml::to_string(&user_config).unwrap();

                    file.write_all(&toml.into_bytes())
                        .expect("Cannot write to the file :(");
                },
        };

        dbg!(config_dir);
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }


}

fn create_config() -> Config {//TODO: Make it automatic rather than requiring user input
    println!("What is the primary package manager that you use?");
    let mut main_pkg_mgr = String::new();
    io::stdin()
            .read_line(&mut main_pkg_mgr)
            .expect("Failed to read line");
    main_pkg_mgr.pop();

    let mut secondary_pkg_mgr = Vec::new();
    match yes_no_prompt() {
        false => println!("Writing Config File..."),
        true => loop {
                println!("What is another package manager that you use?\n (i.e. pacman, apt, portage, npm)");
                let mut pkg_mgr = String::new();
                io::stdin()
                    .read_line(&mut pkg_mgr)
                    .expect("Failed to read line");
                pkg_mgr.pop();
                secondary_pkg_mgr.push(String::from(pkg_mgr.as_str()));
                match yes_no_prompt() {
                    false => {
                        println!("Writing Config File...");
                        break;
                        },
                    true => continue,
                };
            },
    };

    set_config(main_pkg_mgr, secondary_pkg_mgr)



    /*let yn_prompt: char = match guess.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };*/


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

fn set_config (main_pkg_mgr: String, pkg_mgr: Vec<String>) ->Config {
    let mut config = Config {
        current: main_pkg_mgr,
        pacman: false,
        apt: false,
        dnf: false,
        portage: false,
        zypper: false,
        snap: false,
        flatpak: false,
        aur: false,
        npm: false,
        pip: false,
    };
    for element in pkg_mgr {

        if element.eq("pacman") {
            config.pacman = true;
        }
        else if element.eq("apt") {
            config.apt = true;
        }
        else if element.eq("dnf") {
            config.dnf = true;
        }
        else if element.eq("portage") {
            config.portage = true;
        }
        else if element.eq("zypper") {
            config.zypper = true;
        }
        else if element.eq("snap") {
            config.snap = true;
        }
        else if element.eq("flatpak") {
            config.flatpak = true;
        }
        else if element.eq("aur") {
            config.aur = true;
        }
        else if element.eq("npm") {
            config.npm = true;
        }
        else if element.eq("pip") {
            config.pip = true;
        }
        else {
            println!("invalid package manager {}", element);
        }
    }
    config
}

fn check_input (pkg_mgr: &String) -> bool {
    if pkg_mgr.eq("pacman") {
        true
    }
    else if pkg_mgr.eq("apt") {
        true
    }
    else if pkg_mgr.eq("dnf") {
        true
    }
    else if pkg_mgr.eq("portage") {
        true
    }
    else if pkg_mgr.eq("zypper") {
        true
    }
    else if pkg_mgr.eq("snap") {
        true
    }
    else if pkg_mgr.eq("flatpak") {
        true
    }
    else if pkg_mgr.eq("aur") {
        true
    }
    else if pkg_mgr.eq("npm") {
        true
    }
    else if pkg_mgr.eq("pip") {
        true
    }
    else {
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
