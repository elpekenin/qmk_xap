use crate::{
    user::{
        gui::{self, HSV_BLACK, HSV_WHITE},
        UserData,
    },
    xap::hid::XAPDevice,
};
use sysinfo::{CpuExt, SystemExt};
use xap_specs::protocol::painter::HSVColor;

const GAP: u16 = 10;
const WIDTH: u16 = 50;
const FONT: u8 = 1;
const FONT_SIZE: u16 = gui::FONT_SIZES[FONT as usize];
const SCREEN_ID: u8 = 1;
const FIRST_BAR: u16 = 2 * GAP;
const SECOND_BAR: u16 = 3 * GAP + WIDTH;
const DRAW_TEXT: bool = true;
const BAR_DIV: u16 = 2;

fn draw(device: &XAPDevice, bottom: u16, new_value: u8, old_value: u8, is_cpu: bool) {
    // Clear previous bar
    let left = if is_cpu { SECOND_BAR } else { FIRST_BAR };
    let top = bottom - (100 / BAR_DIV) - FONT_SIZE;
    gui::draw::rect(
        device,
        SCREEN_ID,
        left,
        top,
        left + WIDTH,
        bottom + old_value as u16,
        HSV_BLACK,
        true,
    );

    // Draw new bar
    let x = left + WIDTH / 2;
    let y = bottom - new_value as u16 / BAR_DIV;
    let hue = match new_value {
        0..=30 => 105,
        31..=70 => 45,
        _ => 0,
    };
    gui::draw::rect(
        device,
        SCREEN_ID,
        left,
        y,
        left + WIDTH,
        bottom,
        HSVColor {
            hue,
            sat: 255,
            val: 100,
        },
        true,
    );

    // Draw texts
    if DRAW_TEXT {
        gui::draw::centered_text(
            device,
            SCREEN_ID,
            x,
            bottom,
            FONT,
            if is_cpu { "CPU" } else { "RAM" },
        );
        gui::draw::centered_text(
            device,
            SCREEN_ID,
            x,
            y - FONT_SIZE,
            FONT,
            format!("{new_value}%"),
        );
    }
}

pub fn stats(device: &XAPDevice, user_data: &mut UserData) {
    user_data.sys.refresh_all();
    let ram = (100_f64 * user_data.sys.used_memory() as f64
        / (user_data.sys.total_memory() + 1) as f64) as u8;
    let cpu = user_data.sys.global_cpu_info().cpu_usage() as u8;

    let geometry = gui::draw::geometry(device, SCREEN_ID);
    let bottom = geometry.height - FONT_SIZE - GAP;

    if ram != user_data.ram {
        draw(device, bottom, ram, user_data.ram, false);
        user_data.ram = ram;
    }

    if cpu != user_data.cpu {
        draw(device, bottom, cpu, user_data.cpu, true);
        user_data.cpu = cpu;
    }

    // Horizontal line
    gui::draw::rect(
        device,
        SCREEN_ID,
        GAP,
        bottom,
        SECOND_BAR + WIDTH + GAP,
        bottom,
        HSV_WHITE,
        true,
    );
}
