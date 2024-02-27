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

//! user service

pub use repository;

pub fn create_user(id: u64) -> repository::user_repository::entity::user::User {
    let user_entity = repository::user_repository::entity::create_user(id);
    repository::user_repository::save_user(user_entity)
}

pub fn select_one(id: u64) -> repository::user_repository::dto::user_dto::UserDTO {
    repository::user_repository::find_one(id)
}
