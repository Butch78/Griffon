use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;

use sqlx::FromRow;

use crate::expansion::Expansion;
use crate::MyState;

use crate::client::TwitterClient;
use crate::defaults::{Error, Tweet, Tweets, TweetsError};
use shuttle_service::SecretStore;

async fn get_client(state: &State<MyState>) -> Result<TwitterClient, BadRequest<String>> {
    let bearer_token = state.pool.get_secret("bearer_token").await;

    Ok(TwitterClient::builder()
        .set_bearer_token(bearer_token.unwrap())
        .build()
        .unwrap())
}

#[get("/<query>")]
pub async fn get_recent_tweets(
    query: &str,
    state: &State<MyState>,
) -> Result<Json<TweetsError>, BadRequest<String>> {
    let twitter_client = get_client(state).await?;

    let media_expansion = Expansion::User(&["description", "created_at", "location"]);
    let tweet_expansion = Expansion::Tweet(&[
        "author_id",
        "created_at",
        "in_reply_to_user_id",
        "referenced_tweets",
    ]);
    let resp = twitter_client
        .search_recent_tweets("#nyc")
        .expansion(&[media_expansion, tweet_expansion])
        .send();

    match resp {
        Ok(tweets) => Ok(Json(tweets)),
        Err(err) => Err(BadRequest(Some("Bearer token not found".to_string()))),
    }
}
