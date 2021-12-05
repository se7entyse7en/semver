use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.subcommand_matches("hello").is_some() {
        semver::hello();
        std::process::exit(0);
    } else if let Some(matches) = matches.subcommand_matches("validate") {
        if let Some(version) = matches.value_of("version") {
            match semver::validate::validate(version) {
                true => {
                    println!("Version '{}' is valid!", version);
                    std::process::exit(0);
                }
                false => {
                    println!("Version '{}' is not valid!", version);
                    std::process::exit(1);
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("next") {
        if let Some(current_version) = matches.value_of("current-version") {
            if let Some(part) = matches.value_of("part") {
                match semver::next::next(current_version, part) {
                    Ok(version) => {
                        println!("Next version: '{}'", version);
                        std::process::exit(0);
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        std::process::exit(1);
                    }
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("bump") {
        let current_version = matches.value_of("current-version").unwrap();
        let part = matches.value_of("part").unwrap();
        let file = matches.value_of("file").unwrap();
        match semver::bump::bump(current_version, part, file) {
            Ok(version) => {
                println!("Bumped to version: '{}'", version);
                std::process::exit(0);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }
}
