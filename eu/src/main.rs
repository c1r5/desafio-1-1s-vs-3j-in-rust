use types::{UserClient, UserDB};

mod controllers;
mod types;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build().manage::<UserDB>(UserClient::new()).mount(
        "/",
        routes![
            controllers::index,
            controllers::users::upload_users,
            controllers::users::get_superusers,
            controllers::users::get_topcountries
        ],
    )
}
