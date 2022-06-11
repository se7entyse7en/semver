use clap::Args;

#[derive(Args)]
pub struct ValidateArgs {
    /// Version to check
    pub version: String,
}
