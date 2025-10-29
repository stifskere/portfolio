use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::json;


pub fn json_transformer(mut response: HttpResponseBuilder, display: String) -> HttpResponse {
    response
        .json(json!({
            "error": display
        }))
}
