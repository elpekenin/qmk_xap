use crate::{user::gui::draw, xap::hid::XAPDevice, UserData};
use image::{self, ImageBuffer, Rgb};
use log::{debug, error, info, warn};
use parking_lot::lock_api::RawMutex;
use parking_lot::Mutex;
use reqwest;
use rspotify::{
    model::{AdditionalType, Country, FullTrack, Market, PlayableItem},
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token, DEFAULT_CACHE_PATH,
};
use std::sync::Arc;
use xap_specs::protocol::painter::HSVColor;

fn init() -> AuthCodeSpotify {
    let config = Config {
        token_cached: true,
        ..Default::default()
    };

    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

    let creds = Credentials::from_env().unwrap();

    AuthCodeSpotify::with_config(creds, oauth, config)
}

pub fn login() {
    let spotify = init();

    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).unwrap();
}

fn refresh_token(token: Token) -> AuthCodeSpotify {
    match token.is_expired() {
        true => {
            let spotify = init();

            *spotify.token.lock().unwrap() = Some(token);

            match spotify.refresh_token() {
                Err(e) => error!("Failed to refresh token {}", e),
                Ok(_) => debug!("Refreshed token succesfully"),
            }

            spotify
        }
        false => AuthCodeSpotify::from_token(token),
    }
}

fn playing_track(spotify: &AuthCodeSpotify) -> Option<FullTrack> {
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];

    let playing_context = spotify
        .current_playing(Some(market), Some(&additional_types))
        .unwrap();

    let playing_item = match playing_context {
        Some(context) => match context.item {
            Some(item) => item,
            None => {
                debug!("Couldn't get item from context");
                return None;
            }
        },
        None => {
            debug!("Not listening to music");
            return None;
        }
    };

    match playing_item {
        PlayableItem::Track(t) => Some(t),
        PlayableItem::Episode(_) => None,
    }
}

pub fn album_cover(device: &XAPDevice, user_data: &Arc<Mutex<UserData>>) {
    info!("album_cover: in");
    let token = match Token::from_cache(DEFAULT_CACHE_PATH) {
        Ok(t) => t,
        Err(_) => {
            error!("Can't get token from cache");
            return;
        }
    };

    let spotify = refresh_token(token);

    let track = match playing_track(&spotify) {
        Some(track) => track,
        None => {
            debug!("No functionality implemented for podcasts");
            return;
        }
    };

    let song = track.name;

    if user_data.lock().last_song == song {
        debug!("Same song, quitting");
        return;
    } else {
        user_data.lock().last_song = song;
    }

    let artist = &track.artists.first().unwrap().name;
    let url = &track.album.images.last().unwrap().url;

    let img_bytes = match reqwest::blocking::get(url) {
        Ok(response) => response.bytes().unwrap(),
        Err(_) => {
            error!("Couldn't get image for url {}", url);
            return;
        }
    };

    let image = image::load_from_memory(&img_bytes).unwrap();

    let buffer = match image.as_rgb8() {
        Some(b) => b,
        None => {
            error!("Cant convert image to rgb8");
            return;
        }
    };

    info!("album_cover: loop");
    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let pixel = buffer.get_pixel(x, y);

            let image::Rgb([r, g, b]) = pixel;

            println!("Drawing {},{},{} at ({}, {})", r, g, b, x, y);

            draw::pixel(
                device,
                0,
                x as u16,
                y as u16,
                HSVColor::from_rgb(*r, *g, *b),
            );
        }
    }

    info!("album_cover: out");
}
