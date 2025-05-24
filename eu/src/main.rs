mod controllers;
mod types;

use types::{UserClient, UserDB};

#[macro_use] extern crate rocket;


#[launch]
async fn rocket() -> _ {

    rocket::build()
    .manage::<UserDB>(UserClient::new())
    .mount(
        "/", 
        routes![
            controllers::index,
            controllers::users::upload_users
        ]
)
}