use std::fmt::format;

use crate::{
    user::{
        gui::{self, HSV_BLACK, HSV_WHITE, FONT_SIZE},
        UserData,
    },
    xap::hid::XAPDevice,
};
use log::info;
use sysinfo::{CpuExt, NetworkExt, NetworksExt, System, SystemExt};
use xap_specs::protocol::painter::HSVColor;

pub fn stats(device: &XAPDevice, user_data: &mut UserData) {
    user_data.sys.refresh_all();
    let ram = 100_f64 * user_data.sys.used_memory() as f64 / (user_data.sys.total_memory() + 1) as f64;
    let cpu = user_data.sys.global_cpu_info().cpu_usage();

    let screen_id = 1;

    let ram_height = ram as u16;
    let cpu_height = cpu as u16;

    let geometry = gui::draw::geometry(device, screen_id);

    let gap = 10;
    let bottom = geometry.height - FONT_SIZE - gap;
    let top = bottom - 100 - FONT_SIZE;
    let width = 50;

    let first_bar = 2 * gap;
    let second_bar = 3 * gap + width;

    // Clear previous bars
    gui::draw::rect(
        device,
        screen_id,
        first_bar,
        top,
        first_bar + width,
        bottom,
        HSV_BLACK,
        true,
    );
    gui::draw::rect(
        device,
        screen_id,
        second_bar,
        top,
        second_bar + width,
        bottom,
        HSV_BLACK,
        true,
    );

    // Draw bars
    let x = first_bar + width / 2;
    let y = bottom - ram_height;
    let hue = match ram_height {
        0..=30 => 105,
        31..=70 => 45,
        _ => 0
    };
    gui::draw::rect(
        device,
        screen_id,
        first_bar,
        y,
        first_bar + width,
        bottom,
        HSVColor{
            hue,
            sat: 255,
            val: 255
        },
        true,
    );
    gui::draw::text_centered_recolor(device, screen_id, x, bottom, 0, HSV_WHITE, HSV_BLACK, "RAM");
    gui::draw::text_centered_recolor(device, screen_id, x, y - FONT_SIZE, 0, HSV_WHITE, HSV_BLACK, format!("{ram:.2}%"));

    let x = second_bar + width / 2;
    let y = bottom - cpu_height;
    let hue = match cpu_height {
        0..=30 => 128,
        31..=70 => 170,
        _ => 240
    };
    gui::draw::rect(
        device,
        screen_id,
        second_bar,
        y,
        second_bar + width,
        bottom,
        HSVColor{
            hue,
            sat: 255,
            val: 255
        },
        true,
    );
    gui::draw::text_centered_recolor(device, screen_id, x, bottom, 0, HSV_WHITE, HSV_BLACK, "CPU");
    gui::draw::text_centered_recolor(device, screen_id, x, y - FONT_SIZE, 0, HSV_WHITE, HSV_BLACK, format!("{cpu:.2}%"));

    gui::draw::rect(device, screen_id, gap, bottom, second_bar + width + gap, bottom, HSV_WHITE, true);
}
