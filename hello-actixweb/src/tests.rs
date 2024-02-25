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

use actix_web::{http::header::ContentType, test, App};

use crate::router::*;

// ----------------------------------------------------------------

#[actix_web::test]
async fn test_hello_get() {
    let app = test::init_service(App::new().service(hello)).await;

    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_echo_post() {
    let app = test::init_service(App::new().service(echo)).await;

    let req = test::TestRequest::post()
        .uri("/echo")
        .insert_header(ContentType::plaintext())
        .set_payload("Hello world!")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_echo_post_failed() {
    let app = test::init_service(App::new().service(echo)).await;

    let req = test::TestRequest::post()
        .uri("/") // bad path: /echo
        .insert_header(ContentType::plaintext())
        .set_payload("Hello world!")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
}
