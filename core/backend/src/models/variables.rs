#[cfg(not(target_arch = "wasm32"))]
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use bincode::{encode_to_vec, decode_from_slice};
use bincode::config::standard as standard_config;

use crate::models::{ModelResult, ModelError};
use crate::db;


#[derive(Serialize, Deserialize)]
pub struct Variable {
    #[serde(skip_serializing, skip_deserializing)]
    #[expect(dead_code)]
    id: i32,
    var_key: String,
    var_val: Vec<u8>
}

#[cfg(not(target_arch = "wasm32"))]
impl Variable {
    // Returns the number of rows affected.
    async fn query_store<T: Encode>(key: &str, value: T, statement: &str) -> ModelResult<u64> {
        let affected_rows = query(statement)
            .bind(key)
            .bind(
                encode_to_vec(value, standard_config())
                    .map_err(|error| ModelError::Other(error.to_string()))?
            )
            .execute(db!()?)
            .await?
            .rows_affected();

        Ok(affected_rows)
    }

    pub async fn store_or_ignore<T: Encode>(key: &str, value: T) -> ModelResult<bool> {
        Self::query_store(
            key,
            value,
            r"
                INSERT INTO variables (var_key, var_val)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
            "
        )
            .await
            .map(|result| result > 0)
    }

    pub async fn store_or_update<T: Encode>(key: &str, value: T) -> ModelResult<()> {
        Self::query_store(
            key,
            value,
            r"
                INSERT INTO variables (var_key, var_val)
                VALUES ($1, $2)
                ON CONFLICT (var_key) DO UPDATE
                    SET var_val = $2
            "
        )
            .await
            .map(|_| ())
    }

    pub async fn fetch<T: Decode<()>>(key: &str) -> ModelResult<Option<T>> {
        Ok(
            query_as!(
                Self,
                r"
                    SELECT *
                    FROM variables
                    WHERE var_key = $1
                ",
                key
            )
                .fetch_optional(db!()?)
                .await?
                .map(|row| decode_from_slice(&row.var_val, standard_config()))
                .transpose()
                .map_err(|error| ModelError::Other(error.to_string()))?
                .map(|(value, _)| value)
        )
    }
}
