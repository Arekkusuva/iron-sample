
pub fn get_validation_message(field_name: &str) -> &'static str {
    match field_name {
        "email" => "must be email address",
        "password" => "must have at least 7 characters",
        _ => "Validation failed",
    }
}
