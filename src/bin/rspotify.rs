use anyhow::{Context, Result};

use display_switch::configuration::Configuration;
use display_switch::spotify::SpotifyController;
fn main() -> Result<()> {
    let config = Configuration::load().context("failed to load configuration")?;
    let spotify = SpotifyController::new(
        &config.spotify.spotify_client_id,
        &config.spotify.spotify_client_secret,
        &config.spotify.spotify_redirect_uri,
    );

    let _ = spotify.new_raw_spotify()?; // get auth if not exist

    Ok(())
}
