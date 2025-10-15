use actix_web::{web::scope, Scope};
use serde::{Deserialize, Serialize};


pub fn variables_scope() -> Scope {
    scope("/variables")
}

#[derive(Serialize, Deserialize)]
struct Presentation {
    
}

