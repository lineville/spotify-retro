use std::{thread::sleep, time::Duration};

use clap::Parser;
use rspotify::{
    model::{idtypes, FullPlaylist, SearchResult, SearchType},
    prelude::*,
    scopes, AuthCodePkceSpotify, ClientError, Credentials, OAuth,
};

pub use idtypes::*;
use rspotify_model::PlayableId;

const CLIENT_ID: &str = "f9f9113afea14d8698a40a4822d056c2";
const CALLBACK_URL: &str = "http://localhost:8888/callback";

// CLI Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Sprint Theme
    #[clap(short, long, value_parser)]
    sprint_theme: String,

    // Number of songs to add to the playlist
    #[clap(short, long, value_parser)]
    total_songs: Option<u32>,
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    std::env::set_var("RSPOTIFY_CLIENT_ID", CLIENT_ID);
    std::env::set_var("RSPOTIFY_REDIRECT_URI", CALLBACK_URL);

    // Parse CLI Arguments
    let args = Args::parse();
    let theme = args.sprint_theme;
    let total_songs = args.total_songs.unwrap_or(20);

    // Authorize the client
    let client = authorize_client().await.unwrap();

    // Create a new playlist
    let new_playlist_name = format!("{} Sprint", theme);
    let playlist = create_playlist(&client, &new_playlist_name, &new_playlist_name).await;

    // Get tracks
    let tracks = get_tracks(&client, &theme, total_songs).await;

    // Add tracks to the playlist
    let playlist_output = populate_playlist(&client, &playlist, &tracks).await;

    // Print the playlist link and open it in the browser or print error
    match playlist_output {
        Ok(output) => {
            println!("\n\n{}", output);
            open::that(&playlist.external_urls["spotify"]).unwrap();
        }
        Err(err) => {
            println!("\n\n{}", err);
        }
    }
}

// Setup and authorize the client
async fn authorize_client() -> Result<AuthCodePkceSpotify, ClientError> {
    println!("You are about to be redirected to your browser to authenticate with Spotify");
    println!("Copy the URL that you are redirected to and paste it back here!");

    sleep(Duration::from_secs(3));

    let credentials = Credentials::from_env().unwrap();

    let oauth =
        OAuth::from_env(scopes!("playlist-modify-private", "playlist-modify-public")).unwrap();

    let mut client = AuthCodePkceSpotify::new(credentials.clone(), oauth.clone());

    let url = client.get_authorize_url(None).unwrap();
    client.prompt_for_token(&url).await.unwrap();

    // TODO standup a server at the callback URL with a simple landing page
    // Landing page will just have a friendly message and button to copy the URL

    Ok(client)
}

// Create a new playlist
async fn create_playlist(
    client: &AuthCodePkceSpotify,
    name: &str,
    description: &str,
) -> FullPlaylist {
    let user_id = client.current_user().await.unwrap().id;

    client
        .user_playlist_create(&user_id, name, Some(false), Some(true), Some(description))
        .await
        .expect("Failed to create new playlist")
}

// Searches for limit many tracks with the given query
async fn get_tracks(client: &AuthCodePkceSpotify, query: &str, limit: u32) -> SearchResult {
    client
        .search(query, &SearchType::Track, None, None, Some(limit), None)
        .await
        .expect("Failed to find songs to add to playlist")
}

// Adds the given tracks to the given playlist
async fn populate_playlist(
    client: &AuthCodePkceSpotify,
    playlist: &FullPlaylist,
    tracks: &SearchResult,
) -> Result<String, ClientError> {
    match tracks {
        SearchResult::Tracks(tracks) => {
            // Clone and sort tracks by popularity
            let mut tracks = tracks.items.clone();
            tracks.sort_by(|a, b| b.popularity.cmp(&a.popularity));

            let track_ids = tracks
                .iter()
                .map(|t| t.id.as_ref().unwrap() as &dyn PlayableId)
                .collect::<Vec<&dyn PlayableId>>();

            // Add the tracks to the playlist
            client
                .playlist_add_items(&playlist.id, track_ids, None)
                .await
                .expect("Failed to add tracks to playlist");
        }
        _ => {}
    }
    Ok(format!(
        "??? {} playlist ???? {}",
        &playlist.name, playlist.external_urls["spotify"]
    ))
}
