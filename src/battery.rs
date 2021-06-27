use crate::common::{BATTERY_STATE_LABEL, CHARGING_ICON, BATTER_DISPLAY_LABEL};
use libremarkable::ui_extensions::element::{UIElementWrapper, UIElement};
use libremarkable::framebuffer::cgmath::Point2;
use libremarkable::battery::{percentage, human_readable_charging_status};
use libremarkable::framebuffer::common::color;
use libremarkable::appctx;

pub static mut BATTERY_STATE: Option<String> =  None;
pub static mut BATTERY_DISPLAY: i32 = 0;

pub unsafe fn flush_battery_info(app: &mut appctx::ApplicationContext) {
    let current_state = human_readable_charging_status().unwrap();
    debug!("current battery state is : {:?}, and pre is : {:?}" , current_state ,
           BATTERY_STATE.clone().unwrap_or("not init".to_string()));
    if current_state != BATTERY_STATE.clone().unwrap_or("".to_string()) {
        BATTERY_STATE =  Some(human_readable_charging_status().unwrap());
        if current_state == "Charging" {
            app.add_or_flash(
                BATTERY_STATE_LABEL,
                UIElementWrapper {
                    position: Point2 { x: 1720, y: 120 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Image {
                        name: None,
                        img: image::load_from_memory(CHARGING_ICON).unwrap()
                            .resize(40, 100, image::imageops::Nearest),
                        extra: None,
                    },
                },
            );
        } else {
            app.remove_element(BATTERY_STATE_LABEL);
        }
    }
    let percentage = percentage().unwrap();
    debug!("current battery percentage is : {:?}" , percentage);
    if percentage != BATTERY_DISPLAY {
        BATTERY_DISPLAY = percentage;
        app.add_or_flash(
            BATTER_DISPLAY_LABEL,
            UIElementWrapper {
                position: Point2 { x: 1770, y: 155 },
                refresh: Default::default(),
                last_drawn_rect: None,
                onclick: None,
                inner: UIElement::Text {
                    text: BATTERY_DISPLAY.to_string() + "%",
                    scale: 35.0,
                    foreground: color::BLACK,
                    border_px: 0,
                },
            },
        );
    }
}