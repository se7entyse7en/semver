#[cfg(test)]
mod tests;
use crate::{config, core, template};
use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug)]
pub enum FileBumpError {
    Io(io::Error),
    NoOp(String),
}

impl From<io::Error> for FileBumpError {
    fn from(err: io::Error) -> FileBumpError {
        FileBumpError::Io(err)
    }
}

impl PartialEq for FileBumpError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileBumpError::Io(_), _) => false,
            (_, FileBumpError::Io(_)) => false,
            (FileBumpError::NoOp(m1), FileBumpError::NoOp(m2)) => m1 == m2,
        }
    }
}

pub fn replace_files_contents(
    current_version: &core::Version,
    new_version: &core::Version,
    last_stable_version: Option<&core::Version>,
    files: &HashMap<String, config::FileConfig>,
) -> Result<(), FileBumpError> {
    let mut res: Vec<Result<String, FileBumpError>> = vec![];
    for (file_path, file_config) in files {
        let is_stable = new_version.is_stable();
        let stable_only = file_config.stable_only.unwrap_or(false);
        if is_stable || !stable_only {
            res.push(replace_file_content(
                current_version,
                new_version,
                last_stable_version,
                file_config
                    .search
                    .as_ref()
                    .unwrap_or(&"{current_version}".to_owned()),
                file_config
                    .replace
                    .as_ref()
                    .unwrap_or(&"{new_version}".to_owned()),
                file_path,
            ));
        }
    }

    // TODO: In case of an error in one of the files, they should all be reverted.
    res.into_iter()
        .collect::<Result<Vec<String>, FileBumpError>>()
        .map(|_| ())
}

fn replace_file_content(
    current_version: &core::Version,
    new_version: &core::Version,
    last_stable_version: Option<&core::Version>,
    search: &str,
    replace: &str,
    file_path: &str,
) -> Result<String, FileBumpError> {
    let content = fs::read_to_string(file_path)?;
    let context = template::Context::with_versions(
        current_version.to_string(),
        new_version.to_string(),
        last_stable_version
            .map(|v| v.to_string())
            .unwrap_or_else(|| "".to_string()),
    );
    let replaced_content = template::replace_content(&content, search, replace, &context).unwrap();
    if content == replaced_content {
        Err(FileBumpError::NoOp(format!(
            "Nothing changed in file '{}'",
            file_path
        )))
    } else {
        fs::write(file_path, &replaced_content)?;
        Ok(replaced_content)
    }
}
