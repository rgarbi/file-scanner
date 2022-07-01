use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse};
use chrono::Duration;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub fn from_path_to_uuid(id: &web::Path<String>) -> Result<Uuid, HttpResponse> {
    match Uuid::from_str(id.as_str()) {
        Ok(uuid) => Ok(uuid),
        Err(_) => {
            tracing::error!("Got a malformed UUID");
            Err(HttpResponse::BadRequest().finish())
        }
    }
}

pub fn from_string_to_uuid(id: &str) -> Result<Uuid, HttpResponse> {
    match Uuid::from_str(id) {
        Ok(uuid) => Ok(uuid),
        Err(_) => {
            tracing::error!("Got a malformed UUID");
            Err(HttpResponse::BadRequest().finish())
        }
    }
}

pub fn get_unix_epoch_time_as_seconds() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn get_unix_epoch_time_minus_minutes_as_seconds(minus_minutes: i64) -> u64 {
    let duration = Duration::minutes(minus_minutes).num_seconds();
    get_unix_epoch_time_as_seconds() - (duration as u64)
}

pub fn standardize_email(email: &str) -> String {
    email.to_string().to_lowercase()
}

pub fn generate_random_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(50)
        .collect()
}

#[cfg(test)]
mod tests {
    use actix_web::web::Path;
    use claim::assert_ge;
    use uuid::Uuid;

    use crate::util::{from_path_to_uuid, from_string_to_uuid, generate_random_token, get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds, standardize_email};

    #[test]
    fn a_uuid_is_valid() {
        let uuid = Uuid::new_v4();

        assert_eq!(
            uuid,
            from_path_to_uuid(&Path::try_from(uuid.to_string()).unwrap()).unwrap()
        );

        assert_eq!(uuid, from_string_to_uuid(&uuid.to_string()).unwrap());
    }

    #[quickcheck_macros::quickcheck]
    fn anything_not_a_uuid_is_invalid(invalid_uuid: String) -> bool {
        from_path_to_uuid(&Path::try_from(invalid_uuid).unwrap()).is_err()
    }

    #[quickcheck_macros::quickcheck]
    fn anything_not_a_uuid_is_invalid_from_string(invalid_uuid: String) -> bool {
        from_string_to_uuid(&Path::try_from(invalid_uuid).unwrap()).is_err()
    }

    #[test]
    fn get_unix_epoch_time_as_seconds_works() {
        let secs = get_unix_epoch_time_as_seconds();
        println!("{}", secs);
    }

    #[test]
    fn get_unix_epoch_time_minus_minutes_as_seconds_works() {
        let minus_five_minutes = get_unix_epoch_time_minus_minutes_as_seconds(5);
        assert_ge!(
            (get_unix_epoch_time_as_seconds() + (5 * 60)),
            minus_five_minutes
        );
    }

    #[test]
    fn generate_random_token_works() {
        generate_random_token();
    }

    #[test]
    fn standardize_email_works() {
        standardize_email(generate_random_token().as_str());
    }
}
