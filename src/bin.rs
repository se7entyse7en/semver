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
    }
}
