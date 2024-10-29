use rocket::catch;

#[catch(404)]
pub(crate) fn not_found() -> &'static str {
    "Page not found"
}

#[catch(400)]
pub(crate) fn bad_request() -> &'static str {
    "Bad request"
}
