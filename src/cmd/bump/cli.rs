use super::args::BumpArgs;
use super::bump as do_bump;
use crate::cmd::error;
use crate::cmd::helpers::FinalizeArgs;

pub fn bump(args: &BumpArgs) {
    match args.finalize() {
        Ok(config) => {
            match do_bump(
                &config.current_version,
                &config.part,
                config.new_prerelease,
                config.finalize_prerelease,
                &config.files,
                config.bump_prerelease_func,
            ) {
                Ok(version) => {
                    println!("Bumped to version: '{}'", version);
                    match config.original_config.unwrap().update(&version) {
                        Ok(()) => std::process::exit(0),
                        Err(err) => {
                            println!("Error updating the configuration file: {:?}", err);
                            std::process::exit(2);
                        }
                    };
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        Err(err) => error::handle_args_error(err),
    };
}
