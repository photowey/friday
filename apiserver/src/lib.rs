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

pub use service;

pub mod controller;

pub fn run() {
    let user_id = 1708796368_000_000_000;
    let user_entity = controller::user_controller::save_user(user_id);
    println!("The user.entity info is:{:#?}", user_entity);

    let user_dto = controller::user_controller::select_user(user_entity.get_id());
    println!("The user.dto info is:{:#?}", user_dto);
}
