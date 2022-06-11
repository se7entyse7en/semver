use super::error::ArgumentsError;
use crate::config;
use crate::core;
use js_sandbox::{AnyError, Script};

pub trait FinalizeArgs {
    type FinalizedArgs;

    fn get_config(&self) -> Option<String>;

    fn get_required_args(&self) -> Vec<String>;

    fn finalize_from_config(&self, config: config::Config) -> Self::FinalizedArgs;

    fn finalize_from_self(&self) -> Option<Self::FinalizedArgs>;

    fn finalize(&self) -> Result<Self::FinalizedArgs, ArgumentsError> {
        match self.get_config().as_ref() {
            Some(config_path) => {
                let config = config::Config::from_file(config_path)?;
                Ok(self.finalize_from_config(config))
            }
            None => {
                let required_args = self.get_required_args();
                self.finalize_from_self()
                    .ok_or(ArgumentsError::MissingArguments(required_args))
            }
        }
    }
}

pub fn build_bump_func(code: String) -> Result<Box<dyn core::ExtensionBumpFunc>, AnyError> {
    let func: Box<dyn core::ExtensionBumpFunc> = Box::new(move |version| {
        let mut script = Script::from_string(&code)?;
        script.call("bump", &version).map_err(core::BumpError::from)
    });
    Ok(func)
}
