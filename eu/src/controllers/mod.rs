pub(crate) mod users;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}