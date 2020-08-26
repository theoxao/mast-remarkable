extern crate env_logger;

extern crate libremarkable;

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use chrono::{DateTime, Local, Datelike, Timelike};
use libremarkable::appctx::ApplicationContext;
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::common::*;

use libremarkable::input::gpio::GPIOEvent;
use libremarkable::input::multitouch::MultitouchEvent;
use libremarkable::input::wacom::WacomEvent;
use libremarkable::ui_extensions::element::{UIElement, UIElementWrapper, UIElementHandle};
use mast_remarkable::refresh;
use mast_remarkable::common::*;
use mast_remarkable::refresh::{map_week, show_luni_calendar};
use mast_remarkable::weather::show_weather;
use serde_json::Value;

fn on_button_press(_ctx: &mut ApplicationContext, _event: GPIOEvent) {}

fn on_wacom_input(_ctx: &mut ApplicationContext, _event: WacomEvent) {}

fn on_touch_handler(_ctx: &mut ApplicationContext, _event: MultitouchEvent) {}

fn main() {
    env_logger::init();
    unsafe {
        let mut file = File::open(&Path::new("/home/root/mast.ttf")).unwrap();
        static mut BYTES: Vec<u8> = Vec::<u8>::new();

        let _ = file.read_to_end(&mut BYTES);
        let font_data: &[u8] = BYTES.as_slice();

        let mut app = ApplicationContext::new(on_button_press, on_wacom_input, on_touch_handler, font_data);
        let appref = app.upgrade_ref();
        app.clear(true);
        let dt: DateTime<Local> = Local::now();
        refresh::HOUR = dt.hour();
        refresh::DATE = dt.day();
        app.add_element(
            CLOCK_HOUR,
            UIElementWrapper {
                position: Point2 { x: 330, y: 700 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    foreground: color::BLACK,
                    text: format!("{}", dt.format("%H:")),
                    scale: 650.0,
                    border_px: 0,
                },
            },
        );
        app.add_element(
            CLOCK_MINUTE,
            UIElementWrapper {
                position: Point2 { x: 1140, y: 700 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    foreground: color::BLACK,
                    text: format!("{}", dt.format("%M")),
                    scale: 650.0,
                    border_px: 0,
                },
            },
        );
        app.add_element(
            CLOCK_DATE,
            UIElementWrapper {
                position: Point2 { x: 1500, y: 1200 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    text: format!("{}", dt.format("%d")),
                    scale: 350.0,
                    foreground: color::BLACK,
                    border_px: 0,
                },
            },
        );
        app.add_element(
            CLOCK_WEEK,
            UIElementWrapper {
                position: Point2 { x: 1520, y: 1300 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    text: String::from(map_week(format!("{}", dt.format("%A")).as_str())),
                    scale: 100.0,
                    foreground: color::BLACK,
                    border_px: 0,
                },
            },
        );
        show_weather(appref);
        show_luni_calendar(appref);
        // Draw the scene
        app.draw_elements();


        let clock_thread = std::thread::spawn(move || {
            refresh::refresh(appref, 60 * 1000);
        });

        app.dispatch_events(true, true, true);
        clock_thread.join().unwrap();
    }
}


