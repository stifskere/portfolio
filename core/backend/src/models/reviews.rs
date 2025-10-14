use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::db;
use crate::models::{ModelError, ModelResult};

#[derive(Serialize, Deserialize)]
pub struct PartialReview {
    submited_name: String,
    submited_rating: i32,
    submited_comment: String
}

#[derive(Serialize, Deserialize)]
pub struct Review {
    id: i32,
    link_uuid: String,
    can_edit: bool,
    submited_name: Option<String>,
    submited_rating: Option<i32>,
    submited_comment: Option<String>,
    created_at: NaiveDateTime
}

#[allow(dead_code)]
impl PartialReview {
    #[inline]
    pub fn submited_name(&self) -> &str {
        &self.submited_name
    }

    #[inline]
    pub fn submited_rating(&self) -> i32 {
        self.submited_rating
    }

    #[inline]
    pub fn submited_comment(&self) -> &str {
        &self.submited_comment
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Review {
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


    pub async fn get_all_reviews() -> ModelResult<Vec<Self>> {
        Ok(
            query_as!(Self, r"SELECT * FROM reviews")
                .fetch_all(db!()?)
                .await?
        )
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

    pub async fn submit_review(&mut self, partial: PartialReview) -> ModelResult<()> {
        if partial.submited_rating() < 0 || partial.submited_rating() > 5 {
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
            partial.submited_name(),
            partial.submited_rating(),
            partial.submited_comment()
        )
            .fetch_one(db!()?)
            .await?;

        Ok(())
    }

    pub async fn delete_review(self) -> ModelResult<()> {
        query!(r"DELETE FROM reviews WHERE link_uuid = $1", self.link_uuid)
            .execute(db!()?)
            .await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl Review {
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn link_uuid(&self) -> &str {
        &self.link_uuid
    }

    #[inline]
    pub fn can_edit(&self) -> bool {
        self.can_edit
    }

    #[inline]
    pub fn submited_name(&self) -> Option<&String> {
        self.submited_name.as_ref()
    }

    #[inline]
    pub fn submited_rating(&self) -> Option<i32> {
        self.submited_rating
    }

    #[inline]
    pub fn submited_comment(&self) -> Option<&String> {
        self.submited_comment.as_ref()
    }

    #[inline]
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

}
