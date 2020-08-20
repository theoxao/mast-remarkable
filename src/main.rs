extern crate env_logger;
extern crate libremarkable;
#[macro_use]
extern crate log;

use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Local, Date};
use libremarkable::{appctx, battery, image};
use libremarkable::appctx::ApplicationContext;
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::{FramebufferDraw, FramebufferIO, FramebufferRefresh};
use libremarkable::framebuffer::cgmath;
use libremarkable::framebuffer::cgmath::EuclideanSpace;
use libremarkable::framebuffer::common::*;
use libremarkable::framebuffer::refresh::PartialRefreshMode;
use libremarkable::framebuffer::storage;
use libremarkable::image::GenericImage;
use libremarkable::input::{gpio, InputDevice, multitouch, wacom};
use libremarkable::input::gpio::GPIOEvent;
use libremarkable::input::multitouch::MultitouchEvent;
use libremarkable::input::wacom::WacomEvent;
use libremarkable::ui_extensions::element::{
    UIConstraintRefresh, UIElement, UIElementHandle, UIElementWrapper,
};

use mast_remarkable::weather::{get_weather, show_weather};
use chinese_lunisolar_calendar::{LunisolarDate, ChineseVariant};
use chinese_lunisolar_calendar::chrono::NaiveDate;

fn on_button_press(ctx: &mut ApplicationContext, event: GPIOEvent) {}

fn on_wacom_input(ctx: &mut ApplicationContext, event: WacomEvent) {}

fn on_touch_handler(ctx: &mut ApplicationContext, event: MultitouchEvent) {}


fn loop_update_topbar(app: &mut appctx::ApplicationContext, millis: u64) {
    let time_label = app.get_element_by_name("clock").unwrap();
    loop {
        // Get the datetime
        let dt = Local::now();
        if let UIElement::Text { ref mut text, .. } = time_label.write().inner {
            *text = format!("{}", dt.format("%H:%M"));
        }
        // let tl = time_label.write().inner.clone() as UIElement::Text.text;
        app.flash_element("clock");
        sleep(Duration::from_millis(millis));
    }
}


fn main() {
    env_logger::init();
    unsafe {
        let mut file = File::open(&Path::new("/home/root/mast.ttf")).unwrap();
        static mut BYTES: Vec<u8> = Vec::<u8>::new();

        let _ = file.read_to_end(&mut BYTES);
        // let font_data = file!("/home/root/mast.ttf").as_bytes();
        let font_data: &[u8] = BYTES.as_slice();

        let mut app = ApplicationContext::new(on_button_press, on_wacom_input, on_touch_handler, font_data);
        let appref = app.upgrade_ref();
        app.clear(true);
        let dt: DateTime<Local> = Local::now();

        show_luni_calendar(&mut app);
        app.add_element(
            "clock",
            UIElementWrapper {
                position: Point2 { x: 360, y: 700 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    foreground: color::BLACK,
                    text: format!("{}", dt.format("%H:%M")),
                    scale: 650.0,
                    border_px: 0,
                },
            },
        );
        app.add_element(
            "date",
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
        show_weather(appref);
        app.add_element(
            "week",
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
        // Draw the scene
        app.draw_elements();


        let clock_thread = std::thread::spawn(move || {
            loop_update_topbar(appref, 60 * 1000);
        });

        app.dispatch_events(true, true, true);
        clock_thread.join().unwrap();
    }
}

fn show_luni_calendar(app: &mut appctx::ApplicationContext) {
    let now = LunisolarDate::now().unwrap();
    let mut text = String::new();
    let month = now.get_lunar_month().to_str(ChineseVariant::Simple);
    let day = now.get_lunar_day().to_str();
    text.push_str(month);
    text.push_str(day);
    app.add_element(
        "luni_calendar",
        UIElementWrapper {
            position: Point2 { x: 1500, y: 900 },
            refresh: Default::default(),
            last_drawn_rect: None,
            onclick: None,
            inner: UIElement::Text {
                text,
                scale: 75.0,
                foreground: color::BLACK,
                border_px: 0,
            },
        },
    );
}

pub fn map_week(str: &str) -> &str {
    match str {
        "Sunday" => "星期日",
        "Monday" => "星期一",
        "Tuesday" => "星期二",
        "Wednesday" => "星期三",
        "Thursday" => "星期四",
        "Friday" => "星期五",
        "Saturday" => "星期六",
        _ => "星期八",
    }
}
