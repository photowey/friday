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

#[cfg(test)]
mod tests {
    use apiserver::*;

    #[test]
    fn test_save_user() {
        let user_id = 1708796368_000_000_000;
        let user = controller::user::save_user(user_id);

        assert_eq!(user_id, user.get_id())
    }

    #[test]
    fn test_select_user() {
        let user_id = 1708796368_000_000_000;
        let user = controller::user::select_user(user_id);

        assert_eq!(user_id, user.get_id())
    }
}
