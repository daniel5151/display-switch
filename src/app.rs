//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::{Context, Result};

use crate::configuration::{Configuration, SwitchDirection};
use crate::display_control;
use crate::logging;
use crate::platform::{wake_displays, PnPDetect};
use crate::spotify::SpotifyController;
use crate::usb;

pub struct App {
    config: Configuration,
    spotify: SpotifyController,
}

impl usb::UsbCallback for App {
    #[allow(unused_must_use)]
    fn device_added(&self, device_id: &str) {
        debug!("Detected device change. Added device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!("Monitored device ({:?}) is connected", &self.config.usb_device);
            std::thread::spawn(|| {
                wake_displays().map_err(|err| error!("{:?}", err));
            });

            if let Err(err) = self.spotify.switch(&self.config.spotify.spotify_on_usb_connect) {
                error!("could not switch spotify device: {}", err)
            };

            display_control::switch(&self.config, SwitchDirection::Connect);
        }
    }

    fn device_removed(&self, device_id: &str) {
        debug!("Detected device change. Removed device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!("Monitored device is ({:?}) is disconnected", &self.config.usb_device);

            if let Err(err) = self.spotify.switch(&self.config.spotify.spotify_on_usb_disconnect) {
                error!("could not switch spotify device: {}", err)
            };

            display_control::switch(&self.config, SwitchDirection::Disconnect);
        }
    }
}

impl App {
    pub fn new() -> Result<Self> {
        logging::init_logging().context("failed to initialize logging")?;
        let config = Configuration::load().context("failed to load configuration")?;
        let spotify = SpotifyController::new(
            &config.spotify.spotify_client_id,
            &config.spotify.spotify_client_secret,
            &config.spotify.spotify_redirect_uri,
        );

        Ok(Self { config, spotify })
    }

    pub fn run(self) -> Result<()> {
        display_control::log_current_source();
        let pnp_detector = PnPDetect::new(Box::new(self));
        pnp_detector.detect()?;

        Ok(())
    }
}
