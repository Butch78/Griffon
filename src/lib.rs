#[macro_use]
extern crate rocket;

mod builder;
mod client;
mod defaults;
mod expansion;
mod external;
mod twitter;

pub use client::TwitterClient;
pub use expansion::Expansion;

use shuttle_service::error::CustomError;

use sqlx::{Executor, PgPool};

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

pub struct MyState {
    pool: PgPool,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"See <a href="/twitter/nyc">Example Request</a>"#)
}

#[shuttle_service::main]
async fn rocket(pool: PgPool) -> shuttle_service::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/twitter", routes![twitter::get_recent_tweets])
        .manage(state);

    Ok(rocket)
}
