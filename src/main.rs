use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, ClientResult};
use rspotify::model::SearchType;

#[tokio::main]
async fn main() -> ClientResult<()> {
    let creds = Credentials {
        id: "6c8fbd5e22e849ffbfd85c861d5fa569".to_string(),
        secret: include_str!("../secret").to_string(),
    };

    let oauth = OAuth {
        redirect_uri: "http://localhost:8000/callback".to_string(),
        scopes: scopes!(),
        ..OAuth::default()
    };

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;

    let search = spotify.search(
        "7empest",
        &SearchType::Track,
        None,
        None,
        None,
        None,
    ).await?;
    println!("search = {:#?}", search);

    Ok(())
}