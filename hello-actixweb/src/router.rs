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

use actix_web::{get, post, HttpResponse, Responder};

// ----------------------------------------------------------------

// Try visiting:
// GET http://127.0.0.1:8080
// $ curl -X GET "http://127.0.0.1:8080"
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// Try visiting:
// $ curl -X POST -d "Hello world!" "http://127.0.0.1:8080/echo"
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Try visiting:
// GET http://127.0.0.1:8080/hey
// $ curl -X GET "http://127.0.0.1:8080/hey"
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
