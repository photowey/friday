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

//! user repository

pub use abi::types::dto;
pub use abi::types::entity;

pub fn save_user(image: entity::user::User) -> entity::user::User {
    return image;
}

pub fn find_one(id: u64) -> dto::user_dto::UserDTO {
    return dto::create_user_dto(id);
}
