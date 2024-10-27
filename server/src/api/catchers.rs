use rocket::catch;

#[catch(404)]
pub(crate) fn not_found() -> &'static str {
    "Page not found"
}
