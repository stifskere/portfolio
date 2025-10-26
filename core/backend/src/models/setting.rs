#![cfg(not(target_arch = "wasm32"))]

use std::ops::Deref;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query_as, Error as QueryError};
use bincode::{encode_to_vec, decode_from_slice};
use bincode::error::{EncodeError, DecodeError};
use bincode::config::standard as standard_config;
use thiserror::Error;

/// Errors happening while querying or serializing/deserializing
/// a setting from/into the database.
#[derive(Error, Debug)]
pub enum SettingModelError {
    #[error("Error while encoding setting value, {0:#}")]
    Encode(#[from] EncodeError),

    #[error("Error while decoding setting value, {0:#}")]
    Decode(#[from] DecodeError),

    #[error("Error while querying the database, {0:#}")]
    Query(#[from] QueryError)
}

/// The setting relation representation
/// from the database.
///
/// An instance of this really never gets exposed,
/// see [SettingValue] instead.
#[derive(Serialize, Deserialize)]
pub struct Setting {
    #[serde(skip_serializing, skip_deserializing)]
    #[expect(dead_code)]
    id: i32,
    requires_auth: bool,
    var_key: String,
    var_val: Vec<u8>
}

/// A shared representation from the database.
///
/// Used as a query result for [Setting].
pub struct SettingValue<T> {
    requires_auth: bool,
    value: T
}

impl Setting {
    /// Will store a setting, or ignore (not update) it
    /// if it's already declared.
    pub async fn store_or_ignore<T: Encode>(
        pool: &PgPool,
        requires_auth: bool,
        key: &str,
        value: T
    ) -> Result<bool, SettingModelError> {
        Ok(
            query_as!(
                Self,
                r"
                    INSERT INTO settings (requires_auth, var_key, var_val)
                    VALUES ($1, $2, $3)
                    ON CONFLICT DO NOTHING
                ",
                requires_auth,
                key,
                encode_to_vec(value, standard_config())?,
            )
                .execute(pool)
                .await
                .map(|result| result.rows_affected() > 0)?
        )
    }

    /// Will store or replace the current setting if it
    /// already exists.
    pub async fn store_or_update<T: Encode>(
        pool: &PgPool,
        requires_auth: bool,
        key: &str,
        value: T,
    ) -> Result<(), SettingModelError> {
        query_as!(
            Self,
            r"
                INSERT INTO settings (requires_auth, var_key, var_val)
                VALUES ($1, $2, $3)
                ON CONFLICT (var_key) DO UPDATE
                    SET var_val = $3
            ",
            requires_auth,
            key,
            encode_to_vec(value, standard_config())?
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Will obtain a setting from the database.
    pub async fn fetch<T: Decode<()>>(
        pool: &PgPool,
        key: &str
    ) -> Result<Option<SettingValue<T>>, SettingModelError> {
        Ok(
            query_as!(
                Self,
                r"
                    SELECT *
                    FROM settings
                    WHERE var_key = $1
                ",
                key
            )
                .fetch_optional(pool)
                .await?
                .map(|value| value.try_into())
                .transpose()?
        )
    }
}

impl<T> SettingValue<T> {
    /// Whether accessing the variable
    /// requires authentication.
    #[inline]
    pub fn requires_auth(&self) -> bool {
        self.requires_auth
    }

    /// A reference to the inner value.
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The owned inner value.
    #[inline]
    pub fn into_value(self) -> T {
        self.value
    }
}

/// This implementation converts a Setting to a Settingvalue,
/// mostly used internally in the module, since instances
/// of Setting aren't reqlly exposed.
impl<T: Decode<()>> TryFrom<Setting> for SettingValue<T> {
    type Error = SettingModelError;

    fn try_from(setting: Setting) -> Result<Self, Self::Error> {
        Ok(
            Self {
                requires_auth: setting.requires_auth,
                // We are 100% sure that data is bincoded.
                value: decode_from_slice(&setting.var_val, standard_config())
                    .map(|(value, _)| value)?
            }
        )
    }
}

/// This implementation dereferences to the internal value,
/// implemented for retrocompatibility.
impl<T> Deref for SettingValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
