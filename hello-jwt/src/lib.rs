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

/// ## Example
/// ```json
/// {
///   "sub": "jwt",
///   "authorities": "*/*",
///   "pid": "1709222400000000000",
///   "pit": 1,
///   "role": [
///     "1001"
///   ],
///   "scope": [
///     "web"
///   ],
///   "iss": "https:///photowey.github.io/jwt/issuer",
///   "iat": 1709262671000,
///   "jti": "a42su67q7gdnggfdb7shtbueskvnxluu",
///   "aud": "ceb8447cc4ab78d2ec34cd9f11e4bed2",
///   "xoc": "WEB",
///   "exp": 1740798671000
/// }
/// ```
pub struct Claims {
    sub: String,
    aut: String,
    iss: String,
    jti: String,
    aud: String,
    xid: String,
    xit: String,
    xoc: String,
    role: Vec<String>,
    scope: Vec<String>,
    iat: u64,
    exp: u64,
}
