use std::borrow::{Borrow, BorrowMut};
use std::default::default;
use std::ops::{Index, Range, RangeTo, Rem};

use easy_http_request::DefaultHttpRequest;
use image::DynamicImage;
use libremarkable::appctx;
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::cgmath::Vector2;
use libremarkable::framebuffer::common::color;
use libremarkable::ui_extensions::element::{UIElement, UIElementHandle, UIElementWrapper};
use rand::Rng;

use crate::common::{API_HOST, SWITCH_OFF_ICON, SWITCH_ON_ICON};

pub fn get_control() -> Option<Vec<RemarkableDeviceView>> {
    let response = DefaultHttpRequest::get_from_url_str(API_HOST.to_owned() + "/api/remarkable/list").unwrap().send();
    if let Err(e) = response {
        error!("{:?}", e);
        return None;
    }
    debug!("{:?}", response);

    return Some(serde_json::from_slice(response.unwrap().body.as_slice()).unwrap());
}

pub fn show_control(app: &mut appctx::ApplicationContext) {
    if let Some(list) = get_control() {
        let mut x_os = 400;
        let mut y_os = 1000;
        let mut index = 0;
        for device in list {
            debug!("{:?}", device);
            app.add_or_flash(
                (index.to_string() + "_device:icon").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_os + 10, y: y_os - 120 },
                    inner: UIElement::Image {
                        name: Some((index.to_string() + ":device_icon")),
                        img: image::load_from_memory(if device.param.unwrap().value == "1" { SWITCH_ON_ICON } else { SWITCH_OFF_ICON })
                            .unwrap().resize(100, 100, image::imageops::Nearest),
                    },
                    ..default()
                },
            );
            app.add_or_flash(
                (index.to_string() + "_device:name").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_os + 10, y: y_os },
                    inner: UIElement::Text {
                        text: device.device.clone().unwrap().name.to_string(),
                        scale: 30.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                    ..default()
                },
            );
            app.add_or_flash(
                (index.to_string() + "_device:desc").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_os + 10, y: y_os + 42 },
                    inner: UIElement::Text {
                        text: device.device.clone().unwrap().desc.unwrap().to_string(),
                        scale: 20.0,
                        foreground: color::BLUE,
                        border_px: 0,
                    },
                    ..default()
                },
            );
            app.add_or_flash(
                (index.to_string() + "_device:border").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_os - 40, y: y_os - 140 },
                    inner: UIElement::Region {
                        name: Some(index.to_string() + ":device_border"),
                        size: Vector2 { x: 200, y: 200 },
                        border_color: color::BLACK,
                        border_px: 4,
                    },
                    onclick: Some(device_click),
                    ..default()
                },
            );
            if index == 3 {
                y_os += 280;
                x_os = 400;
            } else {
                x_os += 280;
            }
            index += 1;
        }
    }
}

pub fn device_click(app: &mut appctx::ApplicationContext, handler: UIElementHandle) {
    let appref = app.upgrade_ref();
    for ui in &app.ui_elements {
        let pos = ui.1.read().last_drawn_rect.unwrap();
        match ui.1.read().inner {
            UIElement::Region { ref name, .. } => {
                if handler.read().position == ui.1.read().position {
                    let index = name.as_ref().unwrap().index(RangeTo { end: 1 }).parse::<u8>().unwrap();
                    let ele = appref.get_element_by_name((index.to_string() + "_device:icon").as_str()).unwrap();
                    let mut img_name = None;
                    let element = &handler.read().inner;
                    if let UIElement::Image { ref name, .. } = element {
                        img_name = Some(name)
                    }
                    match ele.write().inner {
                        UIElement::Image { ref mut img, ref mut name } => {
                            if let Some(mut n) = img_name {
                                *name = n.clone();
                            }
                            let random: u32 = rand::thread_rng().gen();
                            let value = random.rem(2);
                            *img = image::load_from_memory(if value == 1 { SWITCH_ON_ICON } else { SWITCH_OFF_ICON })
                                .unwrap().resize(100, 100, image::imageops::Nearest)
                        }
                        _ => {}
                    };
                    appref.flash_element((index.to_string() + "_device:icon").as_str());
                }
            }
            _ => {}
        };
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemarkableDeviceView {
    pub device: Option<DeviceView>,
    pub param: Option<Param>,
    pub sort: i32,

}

impl RemarkableDeviceView {}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceView {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
    pub device_type: String,
    pub icon: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub id: i32,
    pub param_type: String,
    pub value_type: String,
    pub key: String,
    pub desc: Option<String>,
    pub options: String,
    pub value: String,
    pub usage: String,
    pub device_id: i32,
    pub in_id: Option<i32>,
    pub out_id: Option<i32>,
}
