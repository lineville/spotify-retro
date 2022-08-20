use clap::Parser;
use rspotify::{
    model::{idtypes, FullPlaylist, SearchResult, SearchType},
    prelude::*,
    scopes, AuthCodePkceSpotify, ClientError, Credentials, OAuth,
};

pub use idtypes::*;
use rspotify_model::{PlayableId, PlaylistResult};

// CLI Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Sprint Number
    #[clap(short, long, value_parser)]
    sprint_number: u32,

    // Number of songs to add to the playlist
    #[clap(short, long, value_parser)]
    total_songs: Option<u32>,
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse CLI Arguments
    let args = Args::parse();
    let sprint_number = args.sprint_number;
    let total_songs = args.total_songs.unwrap_or(20);

    // Authorize the client
    let client = authorize_client().await.unwrap();

    // Create a new playlist
    let new_playlist_name = format!("Sprint {}", sprint_number);
    let playlist = create_playlist(&client, &new_playlist_name, &new_playlist_name).await;

    // Get tracks
    let tracks = get_tracks(&client, &sprint_number.to_string(), total_songs).await;

    // Add tracks to the playlist
    let playlist_output = populate_playlist(&client, &playlist, &tracks).await;

    // Print the playlist output
    match playlist_output {
        Ok(playlist) => {
            println!("\n\n{}", playlist);
        }
        Err(err) => {
            println!("\n\n{}", err);
        }
    }
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
            let tracks = tracks.items.iter();

            let track_ids = tracks
                .map(|t| t.id.as_ref().unwrap() as &dyn PlayableId)
                .collect::<Vec<&dyn PlayableId>>();

            add_tracks_to_playlist(&client, &playlist.id, track_ids).await;
        }
        _ => {}
    }
    Ok(format!(
        "✨ {} playlist 👉 {}",
        &playlist.name, playlist.external_urls["spotify"]
    ))
}

// Add tracks to the playlist
async fn add_tracks_to_playlist(
    spotify: &AuthCodePkceSpotify,
    playlist_id: &PlaylistId,
    tracks: Vec<&dyn PlayableId>,
) -> PlaylistResult {
    spotify
        .playlist_add_items(playlist_id, tracks, None)
        .await
        .expect("Failed to add tracks to playlist")
}

// Create a new playlist
async fn create_playlist(
    spotify: &AuthCodePkceSpotify,
    name: &str,
    description: &str,
) -> FullPlaylist {
    let user_id = spotify.current_user().await.unwrap().id;

    spotify
        .user_playlist_create(&user_id, name, Some(false), Some(true), Some(description))
        .await
        .expect("Failed to create new playlist")
}

// Setup and authorize the client
async fn authorize_client() -> Result<AuthCodePkceSpotify, ClientError> {
    let credentials = Credentials::from_env().unwrap();

    let oauth =
        OAuth::from_env(scopes!("playlist-modify-private", "playlist-modify-public")).unwrap();

    let mut client = AuthCodePkceSpotify::new(credentials.clone(), oauth.clone());

    let url = client.get_authorize_url(None).unwrap();
    client.prompt_for_token(&url).await.unwrap();

    Ok(client)
}
