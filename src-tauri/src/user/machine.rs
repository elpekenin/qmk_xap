use crate::{
    user::{
        gui::{self, FONT_SIZE, HSV_BLACK, HSV_WHITE},
        UserData,
    },
    xap::hid::XAPDevice,
};
use sysinfo::{CpuExt, SystemExt};
use xap_specs::protocol::painter::HSVColor;

const GAP: u16 = 10;
const WIDTH: u16 = 50;
const FONT: u8 = 0;
const SCREEN_ID: u8 = 1;
const FIRST_BAR: u16 = 2 * GAP;
const SECOND_BAR: u16 = 3 * GAP + WIDTH;

fn draw(device: &XAPDevice, bottom: u16, value: u8, cpu: bool) {
    // Clear previous bar
    let left = if cpu { SECOND_BAR } else { FIRST_BAR };
    let top = bottom - 100 - FONT_SIZE;
    gui::draw::rect(
        device,
        SCREEN_ID,
        left,
        top,
        left + WIDTH,
        bottom,
        HSV_BLACK,
        true,
    );

    // Draw new bar
    let x = left + WIDTH / 2;
    let y = bottom - value as u16;
    let hue = match value {
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
            val: 255,
        },
        true,
    );

    // Draw texts
    gui::draw::centered_or_scrolling(
        device,
        SCREEN_ID,
        x,
        bottom,
        FONT,
        if cpu { "CPU" } else { "RAM" },
    );
    gui::draw::centered_or_scrolling(
        device,
        SCREEN_ID,
        x,
        y - FONT_SIZE,
        FONT,
        format!("{value}%"),
    );
}

pub fn stats(device: &XAPDevice, user_data: &mut UserData) {
    user_data.sys.refresh_all();
    let ram = (100_f64 * user_data.sys.used_memory() as f64
        / (user_data.sys.total_memory() + 1) as f64) as u8;
    let cpu = user_data.sys.global_cpu_info().cpu_usage() as u8;

    let geometry = gui::draw::geometry(device, SCREEN_ID);
    let bottom = geometry.height - FONT_SIZE - GAP;

    if ram != user_data.ram {
        user_data.ram = ram;
        draw(device, bottom, ram, false);
    }

    if cpu != user_data.cpu {
        user_data.cpu = cpu;
        draw(device, bottom, cpu, true);
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
