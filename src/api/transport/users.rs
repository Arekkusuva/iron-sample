use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct PostUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "7"))]
    pub password: String,
}
