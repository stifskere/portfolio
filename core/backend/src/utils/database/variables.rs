use std::env::vars;

use crate::models::ModelResult;
use crate::models::variables::Variable;


pub async fn setup_variables() -> ModelResult<()> {
    for (key, value) in vars() {
        if !key.starts_with("PORTFOLIO_VARIABLE_") {
            continue;
        }

        let key = key.trim_start_matches("PORTFOLIO_VARIABLE_");

        let updated = Variable::store_or_ignore(
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
