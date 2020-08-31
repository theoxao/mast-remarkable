use std::env;
use std::fs::FileType;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use image::DynamicImage;
use libremarkable::appctx;
use libremarkable::cgmath::Point2;
use libremarkable::ui_extensions::element::{UIElement, UIElementWrapper, UIElementHandle};

use crate::common;
use crate::wifi::WifiState::{Connected, Enable, Unable};
use libremarkable::appctx::ApplicationContext;

const PATH_ENV: &'static str = "PATH";
const PATH_SYSTEM: &'static str = "/usr/sbin:/sbin";

#[derive(Debug)]
pub enum WifiState {
    Unable(u8),
    Enable(u8),
    Connected(u8),
}

pub fn refresh_wifi_icon(app: &mut appctx::ApplicationContext) {
    let image_bytes = match check_wifi_state() {
        Unable(_) => common::WIFI_OFF_ICON,
        Enable(_) => common::WIFI_ON_ICON,
        Connected(_) => common::WIFI_CONNECTED_ICON,
    };
    app.add_or_flash("wifi_icon",
                     UIElementWrapper {
                         position: Point2 { x: 1760, y: 50 },
                         refresh: Default::default(),
                         last_drawn_rect: None,
                         onclick: None,
                         inner: UIElement::Image {
                             img: image::load_from_memory(image_bytes).unwrap().resize(64, 64, image::imageops::Nearest)
                         },
                     },
    );
}

pub fn turn_on_on_click(app: &mut appctx::ApplicationContext) {
    debug!("handle click wifi icon");
    turn_on();
    refresh_wifi_icon(app);
    sleep(Duration::from_secs(5));
    refresh_wifi_icon(app);
}

pub fn turn_off() {
    debug!("turn wifi off");
    let path = get_path();
    let mut command = Command::new("ifconfig");
    command.env(PATH_ENV, path.clone()).arg("wlan0").arg("down").spawn().unwrap();
}

pub fn turn_on() {
    debug!("turn wifi on");
    let path = get_path();
    let mut command = Command::new("ifconfig");
    command.env(PATH_ENV, path.clone()).arg("wlan0").arg("up").spawn().unwrap();
}

// 0 unable , 1 enable but not connected , 2 connected
pub fn check_wifi_state() -> WifiState {
    let path = get_path();
    let mut x = Command::new("iwconfig");
    let mut command = x
        .env(PATH_ENV, path.clone())
        .arg("wlan0");
    let output = String::from_utf8(command.output().unwrap().stdout).unwrap();
    return if !output.contains("Tx-Power") {
        Unable(0)
    } else {
        if output.contains("ESSID:off/any") {
            Enable(1)
        } else {
            Connected(2)
        }
    };
}

pub fn get_path() -> String {
    env::var_os(PATH_ENV).map_or(PATH_SYSTEM.to_string(), |v| {
        format!("{}:{}", v.to_string_lossy().into_owned(), PATH_SYSTEM)
    })
}
