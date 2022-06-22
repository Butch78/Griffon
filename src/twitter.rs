use rocket::serde::json::Json;

use rocket::{http::Status, response::status, routes, tokio, Build, Rocket, State};
// use sqlx::FromRow;

use crate::expansion::Expansion;
use crate::MyState;

use crate::client::TwitterClient;
use crate::defaults::TweetsError;
use shuttle_service::SecretStore;

use tracing::info;

// async fn get_secret(state: &State<MyState>) -> Result<String, shuttle_service::Error> {
//     info!("Getting secret");
//     // get secret defined in `Secrets.toml` file.
//     Ok(state.pool.get_secret("bearer_token").await?)
// }

fn get_client() -> Result<TwitterClient, shuttle_service::Error> {
    // let _bearer_token = get_secret(&state).await?;

    let temp_brearer_token = "".to_string();

    // Create a new client with the bearer token.
    Ok(TwitterClient::builder()
        .set_bearer_token(temp_brearer_token)
        .build()
        .unwrap())
}

#[get("/<query>")]
pub fn get_recent_tweets(query: &str) -> Result<Json<TweetsError>, status::Custom<String>> {
    let twitter_client = get_client().map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            "Not able to get Twitter Client, sorry 🤷".into(),
        )
    })?;

    let resp = twitter_client.search_recent_tweets(query).send();

    if let Ok(resp) = resp {
        Ok(Json(resp))
    } else {
        Err(status::Custom(
            Status::InternalServerError,
            "Not able to get Tweets, sorry 🤷".into(),
        ))
    }
}

// Create Rust tests
#[cfg(test)]
mod tests {

    // import get_secret function from this module
    use super::{get_client, get_recent_tweets};
    use crate::defaults::TweetsError;
    use crate::MyState;

    #[test]
    fn test_get_client() {
        let client = get_client().unwrap();
        assert_eq!(client.bearer_token, "");
    }

    #[test]
    fn test_get_recent_tweet() {
        let resp = get_recent_tweets("nyc").unwrap();

        // print the response
        

        assert_eq!(1, 1);
    }
}
