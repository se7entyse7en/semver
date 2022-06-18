#[cfg(test)]
mod tests;
use chrono::prelude::{Date, Local, Utc};
use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct Context {
    current_version: String,
    new_version: String,
    last_stable_version: String,
    utc_today_ymd: String,
    local_today_ymd: String,
}

impl Context {
    pub fn with_versions(
        current_version: String,
        new_version: String,
        last_stable_version: String,
    ) -> Self {
        Context::with_versions_and_now(
            current_version,
            new_version,
            last_stable_version,
            Utc::today(),
            Local::today(),
        )
    }

    fn with_versions_and_now(
        current_version: String,
        new_version: String,
        last_stable_version: String,
        utc_today: Date<Utc>,
        local_today: Date<Local>,
    ) -> Self {
        Context {
            current_version,
            new_version,
            last_stable_version,
            utc_today_ymd: utc_today.format("%Y-%m-%d").to_string(),
            local_today_ymd: local_today.format("%Y-%m-%d").to_string(),
        }
    }
}

pub fn replace_content(
    content: &str,
    search: &str,
    replace: &str,
    context: &Context,
) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("search", search)?;
    tt.add_template("replace", replace)?;

    let rendered_search = tt.render("search", context)?;
    let rendered_replace = tt.render("replace", context)?;

    Ok(content.replace(&rendered_search, &rendered_replace))
}
