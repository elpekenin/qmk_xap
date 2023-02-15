use crate::{user::gui::draw, xap::hid::XAPDevice, UserData};
use image;
use log::{debug, error, info, warn};
use reqwest;
use rspotify::{
    model::{AdditionalType, Country, FullTrack, Market, PlayableItem},
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token, DEFAULT_CACHE_PATH,
};
use xap_specs::protocol::painter::HSVColor;
use core::slice::Iter;

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

fn draw_img(device: &XAPDevice, buffer: &mut Iter<u8>, width: u16, height: u16) {
    // iterate backwards due to how data is represented internally by `image`
    for x in (0..width).rev() {
        for y in 0..height {
            // TODO: use `next_chunk` when available
            let r = buffer.next().unwrap();
            let g = buffer.next().unwrap();
            let b = buffer.next().unwrap();

            let color = HSVColor::from_rgb(*r, *g, *b);

            draw::rect(device, 0, 2 * x, 2 * y, 2 * x + 1, 2 * y + 1, color, true);
        }
    }
}

pub(crate) fn album_cover(device: &XAPDevice, user_data: &mut UserData) {
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

    if user_data.last_song == song {
        debug!("Same song, quitting");
        return;
    }

    user_data.last_song = song;

    let artist = &track.artists.first().unwrap().name;
    let url = &track.album.images.last().unwrap().url;

    let img_bytes = match reqwest::blocking::get(url) {
        Ok(response) => response.bytes().unwrap(),
        Err(_) => {
            error!("Couldn't get image from url {}", url);
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

    draw_img(device, &mut buffer.iter(), buffer.width() as u16, buffer.height() as u16);
}
