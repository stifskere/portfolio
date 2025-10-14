use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::db;
use crate::models::{ModelError, ModelResult};

#[derive(Serialize, Deserialize)]
pub struct Reviews {
    id: i32,
    link_uuid: String,
    can_edit: bool,
    submited_name: Option<String>,
    submited_rating: Option<i32>,
    submited_comment: Option<String>,
    created_at: NaiveDateTime
}

impl Reviews {
    pub async fn new_request() -> ModelResult<Uuid> {
        let uuid = Uuid::new_v4();

        query_as!(
            Self,
            r"
                INSERT INTO reviews(link_uuid) VALUES ($1)
                RETURNING *
            ",
            uuid.to_string()
        )
            .fetch_one(db!()?)
            .await?;

        Ok(uuid)
    }

    pub async fn get_review(uuid: Uuid) -> ModelResult<Option<Self>> {
        Ok(
            query_as!(
                Self,
                r"
                    SELECT * FROM reviews
                    WHERE
                        link_uuid = $1
                        AND can_edit = true
                ",
                uuid.to_string()
            )
                .fetch_optional(db!()?)
                .await?
        )
    }

    pub async fn set_editable(&mut self) -> ModelResult<()> {
        if self.can_edit {
            return Ok(());
        }

        self.link_uuid = query!(
            r"
                UPDATE reviews
                SET
                    can_edit = true
                WHERE
                    link_uuid = $1
                RETURNING
                    link_uuid
            ",
            self.link_uuid
        )
            .fetch_one(db!()?)
            .await?
            .link_uuid;

        Ok(())
    }

    pub async fn submit_review(&mut self, name: String, rating: i32, comment: String) -> ModelResult<()> {
        if rating < 0 || rating > 5 {
            return Err(ModelError::Other("Rating must be between 0 to 5.".into()))
        }

        *self = query_as!(
            Self,
            r"
                UPDATE reviews
                SET
                    can_edit = false,
                    submited_name = $2,
                    submited_rating = $3,
                    submited_comment = $4
                WHERE
                    link_uuid = $1
                RETURNING *
            ",
            self.link_uuid,
            name,
            rating,
            comment
        )
            .fetch_one(db!()?)
            .await?;

        Ok(())
    }
}
