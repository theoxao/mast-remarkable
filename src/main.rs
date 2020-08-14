extern crate env_logger;
extern crate libremarkable;
#[macro_use]
extern crate log;

use libremarkable::{appctx, battery, image};
use libremarkable::appctx::ApplicationContext;
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
use libremarkable::cgmath::Point2;
use chrono::{DateTime, Local};
use std::thread::sleep;
use std::time::Duration;

fn on_button_press(ctx: &mut ApplicationContext, event: GPIOEvent) {}

fn on_wacom_input(ctx: &mut ApplicationContext, event: WacomEvent) {}

fn on_touch_handler(ctx: &mut ApplicationContext, event: MultitouchEvent) {}


fn loop_update_topbar(app: &mut appctx::ApplicationContext, millis: u64) {
    let time_label = app.get_element_by_name("time").unwrap();
    let battery_label = app.get_element_by_name("battery").unwrap();
    loop {
        // Get the datetime
        let dt: DateTime<Local> = Local::now();

        if let UIElement::Text { ref mut text, .. } = time_label.write().inner {
            *text = format!("{}", dt.format("%F %r"));
        }

        if let UIElement::Text { ref mut text, .. } = battery_label.write().inner {
            *text = format!(
                "{0:<128}",
                format!(
                    "{0} — {1}%",
                    battery::human_readable_charging_status().unwrap(),
                    battery::percentage().unwrap()
                )
            );
        }
        app.draw_element("time");
        app.draw_element("battery");
        sleep(Duration::from_millis(millis));
    }
}


fn main() {
    env_logger::init();


    let mut app = ApplicationContext::new(on_button_press, on_wacom_input, on_touch_handler);
    app.clear(true);
    app.add_element(
        "logo",
        UIElementWrapper{
            position: Point2 { x: 0, y: 450 },
            refresh: Default::default(),
            last_drawn_rect: None,
            onclick: None,
            inner: UIElement::Text {
                foreground: color::BLACK,
                text: "AAAAAAAAAA".to_owned(),
                scale: 650.0,
                border_px: 2,
            },
        }
    );
    // Draw the scene
    app.draw_elements();
}
