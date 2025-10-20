use serde::{Deserialize, Serialize};
use yew_icons::IconId;
use sqlx::{query, query_as};

use crate::{db, models::{ModelError, ModelResult}};

#[derive(Deserialize)]
pub struct PartialSocial {
    name: String,
    description: Option<String>,
    icon_id: String,
    target: String,
    ui_order: i32
}

#[derive(Serialize)]
pub struct Social {
    #[serde(skip_serializing)]
    id: i32,
    name: String,
    description: Option<String>,
    icon_id: String,
    target: String,
    ui_order: i32
}


impl Social {
    // returns Some if inserted otherwise None.
    pub async fn add(social: PartialSocial) -> ModelResult<Option<Self>> {
        let result = query_as!(
            Self,
            r"
                WITH
                    max_order AS (
                        SELECT COALESCE(MAX(ui_order), 0) AS max_ui_order FROM socials
                    ),
                    shift AS (
                        UPDATE socials
                        SET ui_order = ui_order + 1
                        WHERE
                            ui_order >= $5
                            AND $5 <= (SELECT max_ui_order FROM max_order)
                        RETURNING *
                    )
                INSERT INTO socials (
                    name,
                    description,
                    icon_id,
                    target,
                    ui_order
                )
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    CASE
                        WHEN $5 IS NULL OR $5 > (SELECT max_ui_order FROM max_order)
                            THEN (SELECT max_ui_order + 1 FROM max_order)
                        ELSE $5
                    END
                )
                RETURNING *
            ",
            social.name,
            social.description,
            social.icon_id,
            social.target,
            social.ui_order
        )
            .fetch_optional(db!()?)
            .await?;

        Ok(result)
    }

    // returns Some if updated otherwise None.
    pub async fn edit(id: i32, social: PartialSocial) -> ModelResult<Option<Self>> {
        let result = query_as!(
            Self,
            r"
                UPDATE socials
                SET
                    name = $2,
                    description = $3,
                    icon_id = $4,
                    target = $5,
                    ui_order = COALESCE($6, ui_order)
                WHERE
                    id = $1
                RETURNING *
            ",
            id,
            social.name,
            social.description,
            social.icon_id,
            social.target,
            social.ui_order
        )
            .fetch_optional(db!()?)
            .await?;

        Ok(result)
    }

    // returns true if deleted otherwise false.
    pub async fn delete(id: i32) -> ModelResult<bool> {
        let result = query!(
            r"
                DELETE FROM socials
                WHERE id = $1
            ",
            id
        )
            .execute(db!()?)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
