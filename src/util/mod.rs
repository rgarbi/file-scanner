use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse};
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
    use uuid::Uuid;

    use crate::util::{from_path_to_uuid, from_string_to_uuid, get_unix_epoch_time_as_seconds};

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
        get_unix_epoch_time_as_seconds();
    }
}
