use anyhow::{anyhow, Result};
use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;

pub struct SpotifyController {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl SpotifyController {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        SpotifyController {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
        }
    }

    pub fn new_raw_spotify(&self) -> Result<Spotify> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&self.client_id)
            .client_secret(&self.client_secret)
            .scope("user-read-playback-state user-modify-playback-state")
            .redirect_uri(&self.redirect_uri)
            .build();

        match get_token(&mut oauth) {
            Some(token_info) => {
                let client_credential = SpotifyClientCredentials::default().token_info(token_info).build();
                Ok(Spotify::default().client_credentials_manager(client_credential).build())
            }
            None => Err(anyhow!("failed to auth with spotify")),
        }
    }

    pub fn switch(&self, target: &str) -> Result<()> {
        let spotify = self.new_raw_spotify()?;

        fn switch_inner(spotify: Spotify, target: &str) -> std::result::Result<(), failure::Error> {
            let devices = spotify.device()?;
            let device = devices
                .devices
                .into_iter()
                .find(|v| v.name == target)
                .ok_or_else(|| failure::err_msg(format!("could not find device '{}'", target)))?;

            let playback = spotify.current_playback(None, None)?.ok_or_else(|| failure::err_msg("no current playback"))?;
            spotify.transfer_playback(&device.id, playback.is_playing)?;

            Ok(())
        }

        switch_inner(spotify, target).map_err(|e| anyhow!("{}", e))
    }
}
