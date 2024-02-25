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

//! user dto

#[derive(Debug)]
pub struct UserDTO {
    pub id: u64,
    pub name: String,
    pub age: u16,
    pub account: String,
    pub password: String,
    pub email: String,
    pub nick_name: String,
    pub created_at: u64,
}

impl UserDTO {
    pub fn new(id: u64) -> Self {
        // 1708796368_000_000_000
        return UserDTO {
            id,
            name: "photowey".to_string(),
            age: 18,
            account: "photowey".to_string(),
            password: "photowey9527".to_string(),
            email: "photowey@gmail.com".to_string(),
            nick_name: "photowey".to_string(),
            created_at: 1708796368_000,
        };
    }
}

impl UserDTO {
    pub fn get_id(self) -> u64 {
        self.id
    }
}
