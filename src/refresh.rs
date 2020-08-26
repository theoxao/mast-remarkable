use std::thread::sleep;
use std::time::Duration;

use chrono::{Datelike, Local, Timelike};
use libremarkable::appctx;
use libremarkable::ui_extensions::element::{UIElement, UIElementWrapper};

use crate::common::*;
use chinese_lunisolar_calendar::{LunisolarDate, ChineseVariant};
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::common::color;
use crate::weather::{show_weather, refresh_hourly, refresh_daily};
use std::process::Command;
use serde_json::Value;

pub static mut HOUR: u32 = 25;
pub static mut DATE: u32 = 32;

pub unsafe fn refresh(app: &mut appctx::ApplicationContext, millis: u64) {
    let time_label = app.get_element_by_name(CLOCK_MINUTE).unwrap();
    let hour_label = app.get_element_by_name(CLOCK_HOUR).unwrap();
    let date_label = app.get_element_by_name(CLOCK_DATE).unwrap();
    let week_label = app.get_element_by_name(CLOCK_WEEK).unwrap();
    let dt = Local::now();
    let offset = 60 - dt.second();
    sleep(Duration::from_secs(offset as u64));
    loop {
        // Get the datetime
        let dt = Local::now();
        debug!("{:?}", dt);
        if let UIElement::Text { ref mut text, .. } = time_label.write().inner {
            *text = format!("{}", dt.format("%M"));
        }
        app.flash_element(CLOCK_MINUTE);
        if dt.hour() != HOUR {
            if let UIElement::Text { ref mut text, .. } = hour_label.write().inner {
                *text = format!("{}", dt.format("%H:"))
            };
            HOUR = dt.hour();
            app.flash_element(CLOCK_HOUR);
            refresh_hourly(app)
        }
        if dt.day() != DATE {
            show_luni_calendar(app);
            if let UIElement::Text { ref mut text, .. } = date_label.write().inner {
                *text = format!("{}", dt.format("%d"))
            }
            if let UIElement::Text { ref mut text, .. } = week_label.write().inner {
                *text = map_week(format!("{}", dt.format("%A")).as_str()).to_string()
            }
            app.flash_element(CLOCK_DATE);
            app.flash_element(CLOCK_WEEK);
            refresh_daily(app);
            sync_time();
            DATE = dt.day();
        }
        let dt = Local::now();
        let offset = 60 - dt.second();
        sleep(Duration::from_secs(offset as u64));
    }
}

fn sync_time() {
    let response = easy_http_request::DefaultHttpRequest::get_from_url_str("http://quan.suning.com/getSysTime.do").unwrap().send().unwrap();
    let value: Result<Value, _> = serde_json::from_slice(response.body.as_slice());
    let value1 = value.unwrap();
    let ts = value1.as_object().unwrap().get("sysTime2").unwrap().as_str().unwrap();
    let mut command = Command::new("timedatectl");
    let op = command.arg("set-time").arg(ts);
    debug!("{:?}", op);
    op.spawn().expect("error");
}


pub fn show_luni_calendar(app: &mut appctx::ApplicationContext) {
    let now = LunisolarDate::now().unwrap();
    let mut text = String::new();
    let month = now.get_lunar_month().to_str(ChineseVariant::Simple);
    let day = now.get_lunar_day().to_str();
    text.push_str(month);
    text.push_str(day);
    app.add_or_flash(
        LUNI_DATE,
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