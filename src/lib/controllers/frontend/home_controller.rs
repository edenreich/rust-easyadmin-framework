#[get("/")]
pub fn index() -> &'static str {
    "This is the frontend page"
}