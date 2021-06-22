use std::borrow::Borrow;
use std::env;
use std::fs::FileType;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use image::DynamicImage;
use libremarkable::appctx;
use libremarkable::appctx::ApplicationContext;
use libremarkable::cgmath::Point2;
use libremarkable::ui_extensions::element::{UIElement, UIElementHandle, UIElementWrapper};

use crate::common;
use crate::wifi::WifiState::{Connected, Enable, Unable};

const PATH_ENV: &'static str = "PATH";
const PATH_SYSTEM: &'static str = "/usr/sbin:/sbin";

#[derive(Debug)]
pub enum WifiState {
    Unable(u8),
    Enable(u8),
    Connected(u8),
}

pub fn refresh_wifi_icon(app: &mut appctx::ApplicationContext) {
    let state = check_wifi_state();
    let image_bytes = match state {
        Unable(_) => common::WIFI_OFF_ICON,
        Enable(_) => common::WIFI_ON_ICON,
        Connected(_) => common::WIFI_CONNECTED_ICON,
    };
    let ele = app.get_element_by_name("wifi_icon");
    if ele.is_some() {
        if let UIElement::Image { ref mut name, .. } = ele.unwrap().write().inner {
            // let state = check_wifi_state();
            debug!("origin name: {:?} , current name: {:?}", *name, state);
            if *name.as_ref().unwrap() == format!("{:?}", state) {
                return;
            } else {
                *name = Some(format!("{:?}", state))
            }
        }
    }
    app.add_or_flash("wifi_icon",
                     UIElementWrapper {
                         position: Point2 { x: 1760, y: 50 },
                         refresh: Default::default(),
                         last_drawn_rect: None,
                         onclick: Some(on_wifi_icon_clicked),
                         inner: UIElement::Image {
                             name: Some(format!("{:?}", state)),
                             img: image::load_from_memory(image_bytes).unwrap().resize(64, 64, image::imageops::Nearest),
                             extra: None
                         },
                     },
    );
}


fn on_wifi_icon_clicked(app: &mut appctx::ApplicationContext, _: UIElementHandle) {
    match check_wifi_state() {
        Unable(_) => {
            turn_on_on_click(app);
        }
        Enable(_) => return,  //todo choose a wifi to connect
        Connected(_) => {
            // turn_off();
            // refresh_wifi_icon(app);
        }
    }
}

pub fn turn_on_on_click(app: &mut appctx::ApplicationContext) {
    debug!("handle click wifi icon");
    turn_on();
    refresh_wifi_icon(app);
    wait_util_connected(app);
    refresh_wifi_icon(app);
}

pub fn wait_util_connected(app: &mut appctx::ApplicationContext) {
    for x in 1..15 {
        sleep(Duration::from_secs(1));
        let state = check_wifi_state();
        debug!("{:?}", state);
        match state {
            Unable(_) => turn_on(),
            Enable(_) => {}
            Connected(_) => return,
        }
    }
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
    let mut x = Command::new("iw");
    let mut command = x
        .env(PATH_ENV, path.clone())
        .arg("wlan0").arg("info");
    let output = String::from_utf8(command.output().unwrap().stdout).unwrap();
    return if !output.contains("txpower") {
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
