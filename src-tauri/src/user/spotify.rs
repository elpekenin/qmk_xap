use crate::{
    user::gui::{self, HSV_BLACK},
    xap::hid::XAPDevice,
    UserData,
};
use core::slice::Iter;
use image;
use log::{debug, error, info, trace, warn};
use reqwest;
use rspotify::{
    model::{AdditionalType, Country, FullTrack, Market, PlayableItem},
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token, DEFAULT_CACHE_PATH,
};
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
    if token.is_expired() {
        let spotify = init();

        *spotify.token.lock().unwrap() = Some(token);

        if let Err(e) = spotify.refresh_token() {
            error!("Failed to refresh token {e}");
        } else {
            debug!("Refreshed token succesfully");
        }

        spotify
    } else {
        AuthCodeSpotify::from_token(token)
    }
}

fn playing_track(spotify: &AuthCodeSpotify) -> Option<FullTrack> {
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];

    let playing_context = spotify
        .current_playing(Some(market), Some(&additional_types))
        .unwrap();

    let playing_item = if let Some(context) = playing_context {
        if let Some(item) = context.item {
            item
        } else {
            debug!("Couldn't get item from context");
            return None;
        }
    } else {
        debug!("Not listening to music");
        return None;
    };

    match playing_item {
        PlayableItem::Track(t) => Some(t),
        PlayableItem::Episode(_) => None,
    }
}

pub fn album_cover(device: &XAPDevice, user_data: &mut UserData) {
    let Ok(token) = Token::from_cache(DEFAULT_CACHE_PATH) else {
        error!("Can't get token from cache");
        return;
    };

    let spotify = refresh_token(token);

    let Some(track) = playing_track(&spotify) else {
        debug!("No functionality implemented for podcasts");
        return;
    };

    let song = track.name;

    if user_data.last_song == song {
        trace!("album_cover: same song, quitting");
        return;
    }

    user_data.last_song = song.clone();

    let url = &track.album.images.last().unwrap().url;
    // let url = "https://elpekenin.dev/static/test.png";

    let img_bytes = if let Ok(response) = reqwest::blocking::get(url) {
        response.bytes().unwrap()
    } else {
        error!("Couldn't get image from url {}", url);
        return;
    };

    let target = 0;
    let geometry = gui::draw::geometry(device, target);

    // reset display
    gui::draw::rect(
        device,
        target,
        0,
        0,
        geometry.width - 1,
        geometry.height - 1,
        HSV_BLACK,
        1,
    );

    // draw image
    let image = image::load_from_memory(&img_bytes)
        .unwrap()
        .resize(
            u32::from(geometry.width),
            u32::from(geometry.height),
            image::imageops::FilterType::Nearest,
        )
        .to_rgb8();

    gui::draw::viewport(
        device,
        target,
        0,
        0,
        image.width() as u16 - 1,
        image.height() as u16 - 1,
    );

    image
        .pixels()
        .into_iter()
        .flat_map(|pixel| {
            let image::Rgb([r, g, b]) = pixel;

            // Convert pixels into little endian RGB565 format
            [
                (((r >> 3) & 0x1F) << 3) | ((g >> 5) & 0x07),
                (((g >> 2) & 0x07) << 5) | ((b >> 3) & 0x1F),
            ]
        })
        .collect::<Vec<_>>()
        .chunks(56)
        .for_each(|chunk| gui::draw::pixdata(device, target, chunk));

    let artist = &track.artists.first().unwrap().name;

    gui::draw::surface_text(device, 0, 0, 0, 0, song.as_bytes());
    gui::draw::surface_text(device, 0, 0, gui::FONT_SIZE, 0, artist.as_bytes());
}
