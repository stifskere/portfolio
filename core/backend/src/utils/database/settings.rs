use std::env::vars;

use crate::models::setting::Setting;


pub async fn setup_settings() -> ModelResult<()> {
    for (key, value) in vars() {
        if !key.starts_with("PORTFOLIO_VARIABLE_") {
            continue;
        }

        let key = key.trim_start_matches("PORTFOLIO_VARIABLE_");

        let updated = Setting::store_or_ignore(
            &key,
            &value
        )
            .await?;

        if updated {
            log::info!("Stored variable in the database: {key}={value}");
        }
    }

    Ok(())
}
