use super::args::ValidateArgs;
use super::validate as do_validate;

pub fn validate(args: &ValidateArgs) {
    let version = &args.version;
    match do_validate(version) {
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
