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
#[derive(Debug)]
pub struct Claims {
    sub: String,
    aut: Option<String>,
    iss: Option<String>,
    jti: Option<String>,
    aud: Option<String>,
    xid: Option<String>,
    xit: Option<String>,
    xoc: Option<String>,
    role: Option<Vec<String>>,
    scope: Option<Vec<String>>,
    /// Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub iat: u64,
    pub exp: u64,
}

impl Claims {
    pub fn new(sub: String, iat: u64, exp: u64) -> Self {
        Self {
            sub,
            aut: None,
            iss: None,
            jti: None,
            aud: None,
            xid: None,
            xit: None,
            xoc: None,
            role: Some(Vec::new()),
            scope: Some(Vec::new()),
            iat,
            exp,
        }
    }
}

impl<'a> Claims {
    /// sub getter
    pub fn get_sub(&'a self) -> &'a str {
        self.sub.as_str()
    }
}
