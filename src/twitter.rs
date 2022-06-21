use rocket::serde::json::Json;

use rocket::{
    http::Status,
    response::{status, Redirect},
    routes, Build, Rocket, State,
};
use sqlx::FromRow;

use crate::expansion::Expansion;
use crate::MyState;

use crate::client::TwitterClient;
use crate::defaults::TweetsError;
use shuttle_service::{error::CustomError, SecretStore};

use tracing::info;

async fn get_secret(state: &State<MyState>) -> Result<String, shuttle_service::Error> {
    info!("Getting secret");
    // get secret defined in `Secrets.toml` file.
    Ok(state.pool.get_secret("bearer_token").await?)
}

async fn get_client(state: &State<MyState>) -> Result<TwitterClient, shuttle_service::Error> {
    let bearer_token = get_secret(&state).await?;

    // Create a new client with the bearer token.
    Ok(TwitterClient::builder()
        .set_bearer_token(bearer_token)
        .build()
        .unwrap())
}

#[get("/<query>")]
pub async fn get_recent_tweets(
    query: &str,
    state: &State<MyState>,
) -> Result<Json<TweetsError>, status::Custom<String>> {
    let twitter_client = get_client(state).await.map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            "Not able to get Twitter Client, sorry 🤷".into(),
        )
    })?;

    let media_expansion = Expansion::User(&["description", "created_at", "location"]);
    let tweet_expansion = Expansion::Tweet(&[
        "author_id",
        "created_at",
        "in_reply_to_user_id",
        "referenced_tweets",
    ]);

    let resp = twitter_client
        .search_recent_tweets(query)
        .expansion(&[media_expansion, tweet_expansion])
        .send()
        .map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                "something went wrong, sorry 🤷".into(),
            )
        })?;

    Ok(Json(resp))
}
