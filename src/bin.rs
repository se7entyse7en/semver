use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.subcommand_matches("hello").is_some() {
        semver::hello();
    } else if let Some(matches) = matches.subcommand_matches("validate") {
        match matches.value_of("version") {
            Some(version) => semver::validate::validate(version),
            None => println!("No version provided"),
        }
    }
}
