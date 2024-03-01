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

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};

use jwtt::Claims;

const THOUSANDS_IN_MILLIS: u64 = 1000;

#[test]
fn test_chrono() {
    let now = "2024-03-01 02:03:04";
    let now_timestamp = 1709258584000;
    let dt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let datetime_with_timezone: DateTime<Utc> = Utc.from_utc_datetime(&dt);

    let duration_since_epoch_seconds = datetime_with_timezone.timestamp() as u64;
    let milliseconds = datetime_with_timezone.timestamp_subsec_millis() as u64;
    let duration_since_epoch_millis =
        duration_since_epoch_seconds * THOUSANDS_IN_MILLIS + milliseconds;

    let formatted_string = datetime_with_timezone
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    // 2024-03-01 02:03:04 UTC
    // println!("datetime_with_timezone: {}", datetime_with_timezone);

    assert_eq!(now_timestamp, duration_since_epoch_millis);
    assert_eq!(now, formatted_string);
}

#[test]
fn test_claims() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);
    let iat = to_millis(datetime_utc);
    let exp = to_millis(next_year(datetime_utc));

    let claims = Claims::new("photowey".to_string(), iat, exp);

    assert_eq!("photowey", claims.get_sub());
    assert_eq!(iat, claims.iat);
    assert_eq!(exp, claims.exp);
}

fn to_millis(datetime: DateTime<Utc>) -> u64 {
    let duration_since_epoch_seconds = datetime.timestamp() as u64;
    let milliseconds = datetime.timestamp_subsec_millis() as u64;
    duration_since_epoch_seconds * THOUSANDS_IN_MILLIS + milliseconds
}

fn next_year(datetime: DateTime<Utc>) -> DateTime<Utc> {
    next_year_n(datetime, 1)
}

fn next_year_n(datetime: DateTime<Utc>, n: i32) -> DateTime<Utc> {
    datetime
        .with_year(datetime.year() + n)
        .expect("Invalid DateTime<Utc>")
}
