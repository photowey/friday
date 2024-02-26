/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// ----------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAppPayload {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    id: u64,
    username: String,
}

// ----------------------------------------------------------------

pub fn app() -> Router {
    let router = Router::new();
    register(router)
}

pub fn register(router: Router) -> Router {
    router
        .route("/", get(root))
        .route("/users", post(register_user))
}

// ----------------------------------------------------------------

// GET http://127.0.0.1:3000
// $ curl -X GET "http://127.0.0.1:3000"
pub async fn root() -> &'static str {
    "Hello, World!"
}

// $ curl -X POST -H "Content-Type: application/json" -d "{\"username\":\"photowey\"}" "http://127.0.0.1:3000/users"
pub async fn register_user(Json(payload): Json<UserAppPayload>) -> (StatusCode, Json<UserDTO>) {
    let user = UserDTO {
        id: 9527,
        username: payload.username.clone(),
    };

    (StatusCode::CREATED, Json(user))
}
