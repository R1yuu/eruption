/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::util::RGBA;
use crate::{constants, util};
use gdk::prelude::GdkContextExt;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use palette::{Hsva, Saturate, Shade, Srgba};

type Result<T> = std::result::Result<T, eyre::Error>;

// Key definitions for a generic keyboard with QWERTZ Layout
const KEY_DEFS: &[KeyDef] = &[
    KeyDef::dummy(0), // filler
    // column 1
    KeyDef::new(24.0, 23.0, 30.0, 30.0, "ESC", 1), // ESC
    KeyDef::new(24.0, 76.0, 30.0, 30.0, "^", 2),   // GRAVE_ACCENT
    KeyDef::new(24.0, 110.0, 46.0, 30.0, "TAB", 3), // TAB
    KeyDef::new(24.0, 146.0, 54.0, 30.0, "CAPS LCK", 4), // CAPS_LOCK
    KeyDef::new(24.0, 180.0, 64.0, 30.0, "SHIFT", 5), // SHIFT
    KeyDef::new(24.0, 216.0, 50.0, 30.0, "CTRL", 6), // CTRL
    // column 2
    KeyDef::new(58.0, 76.0, 30.0, 30.0, "1", 7),     // 1
    KeyDef::new(75.0, 110.0, 30.0, 30.0, "Q", 8),    // Q
    KeyDef::new(83.0, 146.0, 30.0, 30.0, "A", 9),    // A
    KeyDef::new(93.0, 180.0, 30.0, 30.0, "<", 10),   // <
    KeyDef::new(78.0, 216.0, 34.0, 30.0, "WIN", 11), // SUPER
    // column 3
    KeyDef::new(88.0, 23.0, 30.0, 30.0, "F1", 12), // F1
    KeyDef::new(91.0, 76.0, 30.0, 30.0, "2", 13),  // 2
    KeyDef::new(109.0, 110.0, 30.0, 30.0, "W", 14), // W
    KeyDef::new(116.0, 146.0, 30.0, 30.0, "S", 15), // S
    KeyDef::new(126.0, 180.0, 30.0, 30.0, "Y", 16), // Y
    KeyDef::new(116.0, 216.0, 32.0, 30.0, "ALT", 17), // ALT
    // column 4
    KeyDef::new(121.0, 23.0, 30.0, 30.0, "F2", 18), // F2
    KeyDef::new(125.0, 76.0, 30.0, 30.0, "3", 19),  // 3
    KeyDef::new(143.0, 110.0, 30.0, 30.0, "E", 20), // E
    KeyDef::new(150.0, 146.0, 30.0, 30.0, "D", 21), // D
    KeyDef::new(159.0, 180.0, 30.0, 30.0, "X", 22), // X
    KeyDef::dummy(23),                              // filler
    // column 5
    KeyDef::new(154.0, 23.0, 30.0, 30.0, "F3", 24), // F3
    KeyDef::new(159.0, 76.0, 30.0, 30.0, "4", 25),  // 4
    KeyDef::new(176.0, 110.0, 30.0, 30.0, "R", 26), // R
    KeyDef::new(184.0, 146.0, 30.0, 30.0, "F", 27), // F
    KeyDef::new(193.0, 180.0, 30.0, 30.0, "C", 28), // C
    // column 6
    KeyDef::new(188.0, 23.0, 30.0, 30.0, "F4", 29), // F4
    KeyDef::new(193.0, 76.0, 30.0, 30.0, "5", 30),  // 5
    KeyDef::new(209.0, 110.0, 30.0, 30.0, "T", 31), // T
    KeyDef::new(218.0, 146.0, 30.0, 30.0, "G", 32), // G
    KeyDef::new(226.0, 180.0, 30.0, 30.0, "V", 33), // V
    // column 7
    KeyDef::new(226.0, 76.0, 30.0, 30.0, "6", 34),  // 6
    KeyDef::new(242.0, 110.0, 30.0, 30.0, "Z", 35), // Z
    KeyDef::new(251.0, 146.0, 30.0, 30.0, "H", 36), // H
    KeyDef::new(259.0, 180.0, 30.0, 30.0, "B", 37), // B
    KeyDef::new(154.0, 216.0, 142.0, 30.0, "SPACE BAR", 38), // SPACE
    // filler
    KeyDef::dummy(39), // filler
    KeyDef::dummy(40), // filler
    KeyDef::dummy(41), // filler
    KeyDef::dummy(42), // filler
    KeyDef::dummy(43), // filler
    KeyDef::dummy(44), // filler
    KeyDef::dummy(45), // filler
    KeyDef::dummy(46), // filler
    KeyDef::dummy(47), // filler
    KeyDef::dummy(48), // filler
    //
    KeyDef::new(233.0, 23.0, 30.0, 30.0, "F5", 49), // F5
    // column 8
    KeyDef::new(259.0, 76.0, 30.0, 30.0, "7", 50),  // 7
    KeyDef::new(275.0, 110.0, 30.0, 30.0, "U", 51), // U
    KeyDef::new(284.0, 146.0, 30.0, 30.0, "J", 52), // J
    KeyDef::new(293.0, 180.0, 30.0, 30.0, "N", 53), // N
    KeyDef::new(266.0, 23.0, 30.0, 30.0, "F6", 54), // F6
    // column 9
    KeyDef::new(292.0, 76.0, 30.0, 30.0, "8", 55),  // 8
    KeyDef::new(308.0, 110.0, 30.0, 30.0, "I", 56), // I
    KeyDef::new(317.0, 146.0, 30.0, 30.0, "K", 57), // K
    KeyDef::new(327.0, 180.0, 30.0, 30.0, "M", 58), // M
    KeyDef::dummy(59),                              // filler
    KeyDef::new(299.0, 23.0, 30.0, 30.0, "F7", 60), // F7
    // column 10
    KeyDef::new(326.0, 76.0, 30.0, 30.0, "9", 61),  // 9
    KeyDef::new(342.0, 110.0, 30.0, 30.0, "O", 62), // O
    KeyDef::new(350.0, 146.0, 30.0, 30.0, "L", 63), // L
    KeyDef::new(360.0, 180.0, 30.0, 30.0, ",", 64), // ,
    KeyDef::dummy(65),                              // filler
    KeyDef::new(333.0, 23.0, 30.0, 30.0, "F8", 66), // F8
    // column 11
    KeyDef::new(359.0, 76.0, 30.0, 30.0, "0", 67),  // 0
    KeyDef::new(375.0, 110.0, 30.0, 30.0, "P", 68), // P
    KeyDef::new(384.0, 146.0, 30.0, 30.0, "Ö", 69), // Ö
    KeyDef::new(394.0, 180.0, 30.0, 30.0, ".", 70), // .
    KeyDef::new(300.0, 216.0, 50.0, 30.0, "ALT GR", 71), // ALT GR
    //
    KeyDef::dummy(72), // filler
    // column 12
    KeyDef::new(393.0, 76.0, 30.0, 30.0, "ß", 73), // ß
    KeyDef::new(408.0, 110.0, 30.0, 30.0, "Ü", 74), // Ü
    KeyDef::new(417.0, 146.0, 30.0, 30.0, "Ä", 75), // Ä
    KeyDef::new(428.0, 180.0, 30.0, 30.0, "-", 76), // -
    KeyDef::new(354.0, 216.0, 50.0, 30.0, "FN", 77), // FN
    //
    KeyDef::dummy(78),                              // filler
    KeyDef::new(380.0, 23.0, 30.0, 30.0, "F9", 79), // F9
    // column 13
    KeyDef::new(426.0, 76.0, 30.0, 30.0, "´", 80), // ´
    KeyDef::new(442.0, 110.0, 30.0, 30.0, "+", 81), // +
    KeyDef::dummy(82),                              // filler
    KeyDef::new(461.0, 180.0, 51.0, 30.0, "SHIFT", 83), // SHIFT
    KeyDef::new(407.0, 216.0, 50.0, 30.0, "MENU", 84), // MENU
    KeyDef::new(412.0, 23.0, 30.0, 30.0, "F10", 85), // F10
    KeyDef::new(445.0, 23.0, 30.0, 30.0, "F11", 86), // F11
    // column 14
    KeyDef::new(478.0, 23.0, 30.0, 30.0, "F12", 87), // F12
    KeyDef::new(459.0, 76.0, 51.0, 30.0, "BKSPC", 88), // BACKSPACE
    KeyDef::new(482.0, 110.0, 30.0, 66.0, "RETURN", 89), // RETURN
    KeyDef::new(460.0, 216.0, 50.0, 30.0, "CTRL", 90), // CTRL
    KeyDef::dummy(91),                               // filler
    KeyDef::dummy(92),                               // filler
    // filler
    KeyDef::dummy(93),                              // filler
    KeyDef::dummy(94),                              // filler
    KeyDef::dummy(95),                              // filler
    KeyDef::dummy(96),                              // filler
    KeyDef::new(450.0, 146.0, 30.0, 30.0, "#", 97), // #
    KeyDef::dummy(98),                              // filler
    KeyDef::dummy(99),                              // filler
    // column 15
    KeyDef::new(524.0, 23.0, 30.0, 30.0, "PRT", 100), // PRINT SCREEN
    KeyDef::new(524.0, 76.0, 30.0, 30.0, "INS", 101), // INSERT
    KeyDef::new(524.0, 110.0, 30.0, 30.0, "DEL", 102), // DELETE
    KeyDef::new(524.0, 216.0, 30.0, 30.0, "←", 103), // LEFT
    // column 15
    KeyDef::new(557.0, 23.0, 30.0, 30.0, "SCRL", 104), // SCROLL LOCK
    KeyDef::new(557.0, 76.0, 30.0, 30.0, "HOME", 105), // HOME
    KeyDef::new(557.0, 110.0, 30.0, 30.0, "END", 106), // END
    KeyDef::new(557.0, 180.0, 30.0, 30.0, "↑", 107), // UP
    KeyDef::new(557.0, 216.0, 30.0, 30.0, "↓", 108), // DOWN
    // column 16
    KeyDef::new(590.0, 23.0, 30.0, 30.0, "PAUSE", 109), // PAUSE
    KeyDef::new(590.0, 76.0, 30.0, 30.0, "PGUP", 110),  // PAGE UP
    KeyDef::new(590.0, 110.0, 30.0, 30.0, "PGDWN", 111), // PAGE DOWN
    KeyDef::new(590.0, 216.0, 30.0, 30.0, "→", 112),  // RIGHT
    KeyDef::dummy(113),                                 // filler
    // column 17
    KeyDef::new(635.0, 76.0, 30.0, 30.0, "NUM", 114), // NUM LOCK
    KeyDef::new(635.0, 110.0, 30.0, 30.0, "7", 115),  // 7
    KeyDef::new(635.0, 146.0, 30.0, 30.0, "4", 116),  // 4
    KeyDef::new(635.0, 180.0, 30.0, 30.0, "1", 117),  // 1
    KeyDef::new(635.0, 216.0, 64.0, 30.0, "0", 118),  // 0
    KeyDef::dummy(119),                               // filler
    // column 18
    KeyDef::new(669.0, 76.0, 30.0, 30.0, "/", 120), // /
    KeyDef::new(669.0, 110.0, 30.0, 30.0, "8", 121), // 8
    KeyDef::new(669.0, 146.0, 30.0, 30.0, "5", 122), // 5
    KeyDef::new(669.0, 180.0, 30.0, 30.0, "2", 123), // 2
    KeyDef::dummy(124),                             // filler
    // column 19
    KeyDef::new(702.0, 76.0, 30.0, 30.0, "*", 125), // *
    KeyDef::new(702.0, 110.0, 30.0, 30.0, "9", 126), // 9
    KeyDef::new(702.0, 146.0, 30.0, 30.0, "6", 127), // 6
    KeyDef::new(702.0, 180.0, 30.0, 30.0, "3", 128), // 3
    KeyDef::new(702.0, 216.0, 30.0, 30.0, ",", 129), // ,
    // column 20
    KeyDef::new(735.0, 76.0, 30.0, 30.0, "-", 130), // -
    KeyDef::new(735.0, 110.0, 30.0, 66.0, "+", 131), // +
    KeyDef::new(735.0, 180.0, 30.0, 66.0, "ENTER", 132), // ENTER
    // filler
    KeyDef::dummy(133), // filler
    KeyDef::dummy(134), // filler
    KeyDef::dummy(135), // filler
    KeyDef::dummy(136), // filler
    KeyDef::dummy(137), // filler
    KeyDef::dummy(138), // filler
    KeyDef::dummy(139), // filler
    KeyDef::dummy(140), // filler
    KeyDef::dummy(141), // filler
    KeyDef::dummy(142), // filler
    KeyDef::dummy(143), // filler
    KeyDef::dummy(144), // filler
];

#[derive(Debug, PartialEq)]
struct KeyDef<'a> {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    caption: &'a str,
    // index: usize,
}

impl<'a> KeyDef<'a> {
    const fn new(x: f64, y: f64, width: f64, height: f64, caption: &'a str, _index: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            caption,
            // index, // currently only included for documentation purposes
        }
    }

    const fn dummy(_index: usize) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            caption: "",
            // index, // currently only included for documentation purposes
        }
    }
}

/// Initialize page "Keyboard"
pub fn initialize_keyboard_page(builder: &gtk::Builder) -> Result<()> {
    let drawing_area: gtk::DrawingArea = builder.get_object("drawing_area").unwrap();

    let networkfx_ambient_switch: gtk::Switch = builder.get_object("soundfx_switch").unwrap();
    let soundfx_switch: gtk::Switch = builder.get_object("soundfx_switch").unwrap();

    // drawing area / keyboard indicator
    drawing_area.connect_draw(&draw_keyboard);

    glib::timeout_add_local(
        1000 / (constants::TARGET_FPS as u32 * 2),
        clone!(@strong drawing_area => move || {
            drawing_area.queue_draw();
            Continue(true)
        }),
    );

    // special options
    networkfx_ambient_switch.connect_state_set(move |_sw, enabled| {
        util::toggle_netfx_ambient(enabled).unwrap();

        gtk::Inhibit(false)
    });

    soundfx_switch.set_state(util::get_sound_fx()?);

    soundfx_switch.connect_state_set(move |_sw, enabled| {
        util::set_sound_fx(enabled).unwrap();

        gtk::Inhibit(false)
    });

    Ok(())
}

/// Paint a key on the keyboard widget
fn paint_key(key: usize, color: &RGBA, cr: &cairo::Context, _scale_factor: f64) {
    let key_def = &KEY_DEFS[key];

    // compute scaling factor
    let factor = ((100.0 - crate::STATE.read().current_brightness.unwrap_or_else(|| 0) as f64)
        / 100.0)
        * 0.05;

    // post-process color
    let color = Srgba::new(
        color.r as f64 / 255.0,
        color.g as f64 / 255.0,
        color.b as f64 / 255.0,
        color.a as f64 / 255.0,
    );

    // saturate and lighten color somewhat
    let color = Hsva::from(color);
    let color = Srgba::from(color.saturate(factor).lighten(factor)).into_components();

    cr.set_source_rgba(color.0, color.1, color.2, 1.0 - color.3);
    cr.rectangle(key_def.x, key_def.y, key_def.width, key_def.height);
    cr.fill();

    cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
    cr.select_font_face(
        "Sans Serif",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Normal,
    );
    cr.move_to(7.0 + key_def.x, 32.0 + (key_def.y - (key_def.height / 2.0)));
    cr.show_text(&key_def.caption);
}

/// Draw an animated keyboard with live action colors
pub fn draw_keyboard<D: IsA<gtk::DrawingArea>>(_da: &D, context: &cairo::Context) -> gtk::Inhibit {
    // let da = da.as_ref();

    // let width = da.get_allocated_width();
    // let height = da.get_allocated_height();

    let scale_factor = 1.5;

    let pixbuf = Pixbuf::from_resource("/org/eruption/eruption-gui/img/keyboard.png").unwrap();

    // paint the schematic drawing
    context.scale(scale_factor, scale_factor);
    context.set_source_pixbuf(&pixbuf, 0.0, 0.0);
    context.paint();

    let led_colors = crate::dbus_client::get_led_colors().unwrap();

    // paint all keys
    for i in 0..144 {
        // if let Some(index) = KEY_DEFS.iter().position(|e| e.index == i) {
        //     paint_key(index + 1, &led_colors[i], &context);
        // } else {
        //     log::error!("Invalid key index: {}", i);
        // }

        paint_key(i + 1, &led_colors[i], &context, scale_factor);
    }

    gtk::Inhibit(false)
}
