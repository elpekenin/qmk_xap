use crate::{
    user::gui::{self, HSV_BLACK, HSV_WHITE},
    xap::hid::XAPDevice,
    UserData,
};
use image;
use log::{debug, error, trace};
use reqwest;
use rspotify::{
    model::{AdditionalType, Country, FullTrack, Market, PlayableItem},
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token, DEFAULT_CACHE_PATH,
};
use xap_specs::protocol::painter::PainterGeometry;

use super::gui::FONT_SIZE;

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

    match spotify
        .current_playing(Some(market), Some(&additional_types))
        .ok()??
        .item
    {
        Some(PlayableItem::Track(t)) => Some(t),
        x => {
            debug!("No logic defined for {:#?}", x);
            None
        }
    }
}

fn draw_album_img(device: &XAPDevice, url: &String, screen_id: u8, geometry: &PainterGeometry) {
    let img_bytes = if let Ok(response) = reqwest::blocking::get(url) {
        response.bytes().unwrap()
    } else {
        error!("Couldn't get image from url {url}");
        return;
    };

    // draw image
    let image = image::load_from_memory(&img_bytes)
        .unwrap()
        .resize(64, 64, image::imageops::FilterType::Nearest)
        .to_rgb8();

    let width = image.width() as u16;
    let height = image.height() as u16;

    let left = (geometry.width - width) / 2;
    let top = (geometry.height - height) / 2;

    gui::draw::viewport(
        device,
        screen_id,
        left,
        top,
        left + width - 1,
        top + height - 1,
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
        .for_each(|pixels| gui::draw::pixdata(device, screen_id, pixels));
}

pub fn album_cover(device: &XAPDevice, user_data: &mut UserData) {
    let Ok(token) = Token::from_cache(DEFAULT_CACHE_PATH) else {
        error!("Can't get token from cache");
        return;
    };

    let spotify = refresh_token(token);

    let screen_id = 0;
    let font = 0;
    let geometry = gui::draw::geometry(device, screen_id);
    let x = geometry.width / 2;
    let gap = (geometry.height - 64) / 2;

    // Guard clause - No song
    let Some(track) = playing_track(&spotify) else {
        gui::draw::stop_scrolling_text(device, user_data.artist_token);
        gui::draw::stop_scrolling_text(device, user_data.song_token);

        if user_data.last_song == "__none__" {
            return;
        }
        gui::draw::clear(device, screen_id);
        gui::draw::text_recolor(
            device,
            screen_id,
            0,
            (geometry.height - FONT_SIZE) / 2,
            font,
            HSV_WHITE,
            HSV_BLACK,
            "No song on Spotify"
        );

        user_data.last_song = String::from("__none__");
        user_data.last_url = Default::default();

        return;
    };

    // Guard clause - Same song
    let song = track.name;
    if user_data.last_song == song {
        trace!("album_cover: same song, quitting");
        return;
    }
    user_data.last_song = song.clone();

    // Stop texts
    gui::draw::stop_scrolling_text(device, user_data.song_token);
    gui::draw::stop_scrolling_text(device, user_data.artist_token);

    // Draw song image, if different
    let url = &track.album.images.last().unwrap().url;
    if &user_data.last_url != url {
        gui::draw::rect(
            device,
            screen_id,
            0,
            gap,
            geometry.width,
            geometry.height - gap,
            HSV_BLACK,
            true,
        );
        draw_album_img(device, url, screen_id, &geometry);
    }
    user_data.last_url = url.to_string();

    // Draw song name
    let song = song.as_bytes();
    let y = geometry.height - gap;
    gui::draw::rect(
        device,
        screen_id,
        0,
        y,
        geometry.width,
        geometry.height,
        HSV_BLACK,
        true,
    );
    user_data.song_token = gui::draw::centered_or_scrolling_text(device, screen_id, y, font, song);

    // Draw artist name
    let artist = track.artists.first().unwrap().name.as_bytes();
    let y = geometry.height / 2 - 32 - FONT_SIZE;
    gui::draw::rect(
        device,
        screen_id,
        0,
        0,
        geometry.width,
        gap,
        HSV_BLACK,
        true,
    );
    user_data.artist_token = gui::draw::centered_or_scrolling_text(device, screen_id, y, font, artist);
}
