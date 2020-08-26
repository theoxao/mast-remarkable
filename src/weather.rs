use std::borrow::BorrowMut;
use std::default::default;

use chrono::{FixedOffset, TimeZone, Utc};
use easy_http_request::DefaultHttpRequest;
use libremarkable::appctx;
use libremarkable::appctx::ApplicationContext;
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::{cgmath, common, FramebufferDraw, FramebufferRefresh};
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, mxcfb_rect, waveform_mode};
use libremarkable::framebuffer::refresh::PartialRefreshMode;
use libremarkable::ui_extensions::element::{UIElement, UIElementHandle, UIElementWrapper};

pub fn get_weather() -> Option<Weather> {
    let response = DefaultHttpRequest::get_from_url_str("http://api.theoxao.com/api/weather").unwrap().send();
    if let Err(e) = response {
        error!("{:?}", e);
        return None;
    }
    Some(serde_json::from_slice(response.unwrap().body.as_slice()).unwrap())
}

static N01_ICON: &[u8] = include_bytes!("../assets/icon/01n@4x.png") as &[u8];
static N02_ICON: &[u8] = include_bytes!("../assets/icon/02n@4x.png") as &[u8];
static N03_ICON: &[u8] = include_bytes!("../assets/icon/03n@4x.png") as &[u8];
static N04_ICON: &[u8] = include_bytes!("../assets/icon/04n@4x.png") as &[u8];
static N09_ICON: &[u8] = include_bytes!("../assets/icon/09n@4x.png") as &[u8];
static N10_ICON: &[u8] = include_bytes!("../assets/icon/10n@4x.png") as &[u8];
static N11_ICON: &[u8] = include_bytes!("../assets/icon/11n@4x.png") as &[u8];
static N13_ICON: &[u8] = include_bytes!("../assets/icon/13n@4x.png") as &[u8];
static N50_ICON: &[u8] = include_bytes!("../assets/icon/50n@4x.png") as &[u8];
static ALERT_ICON: &[u8] = include_bytes!("../assets/icon/alert.png") as &[u8];

static WEATHER: Option<Weather> = None;

pub fn show_weather(app: &mut appctx::ApplicationContext) {
    if let Some(weather) = get_weather() {
        weather.show_current_weather(app);
        weather.show_daily_weather(app);
        weather.show_hourly_weather(app);
    }
}

pub fn refresh_hourly(app: &mut appctx::ApplicationContext) {
    refresh(app)
}

pub fn refresh_daily(app: &mut appctx::ApplicationContext) {
    if let Some(weather) = get_weather() {
        weather.show_daily_weather(app);
    }
}

pub fn refresh(app: &mut appctx::ApplicationContext) {
    if let Some(weather) = get_weather() {
        let fb = app.get_framebuffer_ref();
        let rect = mxcfb_rect::from(Point2 { x: 0, y: 0 }, cgmath::Vector2 { x: 1850, y: 235 });
        fb.fill_rect(rect.top_left().cast().unwrap(), rect.size(), color::WHITE);
        fb.partial_refresh(
            &rect,
            PartialRefreshMode::Wait,
            waveform_mode::WAVEFORM_MODE_DU,
            display_temp::TEMP_USE_AMBIENT,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );
        let eles = &mut app.ui_elements.to_owned();
        // let app: &'static mut ApplicationContext<'static> = app.upgrade_ref();
        for x in eles {
            if x.0.ends_with(":current:weather") || x.0.ends_with(":hourly:weather") {
                let mut lock_ele = x.1.write();
                match lock_ele.inner {
                    UIElement::Text {
                        ref text,
                        scale,
                        foreground,
                        border_px,
                    } => app.display_text(
                        lock_ele.position.cast().unwrap(),
                        foreground,
                        scale,
                        border_px as u32,
                        8,
                        text.to_string(),
                        lock_ele.refresh,
                    ),
                    UIElement::Image { ref img } => {
                        app.display_image(&img, lock_ele.position.cast().unwrap(), lock_ele.refresh)
                    }
                    UIElement::Region {
                        size,
                        border_color,
                        border_px,
                    } => app.display_rect(
                        lock_ele.position.cast().unwrap(),
                        size.cast().unwrap(),
                        border_px,
                        border_color,
                        lock_ele.refresh,
                    ),
                    UIElement::Unspecified => return,
                };

                if let Some(last_rect) = lock_ele.last_drawn_rect {
                    if last_rect != rect {
                        fb.partial_refresh(
                            &last_rect,
                            PartialRefreshMode::Async,
                            common::waveform_mode::WAVEFORM_MODE_DU,
                            common::display_temp::TEMP_USE_REMARKABLE_DRAW,
                            common::dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                            0,
                            false,
                        );
                    }
                }
            }
        }
    }
}

impl Weather {
    pub fn show_current_weather(&self, app: &mut appctx::ApplicationContext) {
        let time_zone = FixedOffset::east(8 * 3600);
        let current = &self.current;
        let weather_desc = current.weather.get(0).unwrap().clone();
        let icon = weather_desc.icon;
        let desc_id = weather_desc.id;
        let img = image::load_from_memory(map_icon(icon)).unwrap()
            .resize(180, 180, image::imageops::Nearest);

        app.add_or_flash(
            "icon:current:weather",
            UIElementWrapper {
                position: Point2 { x: 16, y: 16 },
                onclick: None,
                inner: UIElement::Image {
                    img
                },
                ..default()
            },
        );
        app.add_or_flash(
            "temp:current:weather",
            UIElementWrapper {
                position: Point2 { x: 220, y: 80 },
                inner: UIElement::Text {
                    text: ((current.temp as u8).to_string() + "℃").to_string(),
                    scale: 50.0,
                    foreground: color::BLACK,
                    border_px: 0,
                },
                ..default()
            },
        );
        app.add_or_flash(
            "desc:current:weather",
            UIElementWrapper {
                position: Point2 { x: 220, y: 140 },
                inner: UIElement::Text {
                    text: map_desc(desc_id),
                    scale: 50.0,
                    foreground: color::BLACK,
                    border_px: 0,
                },
                ..default()
            },
        );

        let update_time = format!("{}", Utc.timestamp(current.dt as i64, 0)
            .with_timezone(&time_zone).format("%m-%d %H:%M"));
        debug!("{} , {}", current.dt, update_time);
        app.add_or_flash(
            "update_time:current:weather",
            UIElementWrapper {
                position: Point2 { x: 80, y: 230 },
                inner: UIElement::Text {
                    text: ("更新时间:".to_string() + update_time.as_str()).to_string(),
                    scale: 20.0,
                    foreground: color::BLUE,
                    border_px: 0,
                },
                ..default()
            },
        );
    }

    pub fn show_daily_weather(&self, app: &mut appctx::ApplicationContext) {
        let time_zone = FixedOffset::east(8 * 3600);
        let mut y_offset: i32 = 330;
        let x_offset: i32 = 20;
        let mut i = 0;
        for day in &self.daily {
            let date = format!("{}", Utc.timestamp(day.dt as i64, 0)
                .with_timezone(&time_zone).format("%m月%d日"));
            let min_temp = (day.temp.min as i32).to_string();
            let max_temp = (day.temp.max as i32).to_string();
            let temp = max_temp + "℃/" + min_temp.as_str() + "℃";
            let weather = day.weather.get(0).unwrap();
            let img = image::load_from_memory(map_icon(weather.clone().icon)).unwrap()
                .resize(64, 64, image::imageops::Nearest);
            let desc = map_desc(weather.id);
            app.add_or_flash(
                (i.to_string() + "_date:daily:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: 90 + x_offset, y: y_offset },
                    inner: UIElement::Text {
                        text: date.clone(),
                        scale: 28.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                    ..default()
                },
            );
            app.add_or_flash(
                (i.to_string() + "_temp:daily:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: 20 + x_offset, y: y_offset + 60 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Text {
                        text: temp,
                        scale: 26.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                },
            );
            app.add_or_flash(
                (i.to_string() + "_icon:daily:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: 20 + x_offset, y: y_offset - 40 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Image {
                        img
                    },
                },
            );
            app.add_or_flash(
                (i.to_string() + "_desc:daily:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: 150 + x_offset, y: y_offset + 60 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Text {
                        text: desc,
                        scale: 26.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                },
            );
            i += 1;
            y_offset += 140;
        }
    }

    pub fn show_hourly_weather(&self, app: &mut appctx::ApplicationContext) {
        let mut i = 0;
        let mut x_offset = 350;
        let y_offset = 50;
        let time_zone = FixedOffset::east(8 * 3600);
        for record in &self.hourly {
            if x_offset > 1800 {
                break;
            }
            let temp = (record.temp as i32).to_string();
            let weather = record.weather.get(0).unwrap();
            let hour = format!("{}", Utc.timestamp(record.dt as i64, 0).with_timezone(&time_zone).format("%H时"));
            let img = image::load_from_memory(map_icon(weather.clone().icon)).unwrap()
                .resize(50, 50, image::imageops::Nearest);
            app.add_or_flash(
                (i.to_string() + "_hour:hourly:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_offset, y: y_offset },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Text {
                        text: hour,
                        scale: 26.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                },
            );
            app.add_or_flash(
                (i.to_string() + "_icon:hourly:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_offset, y: y_offset + 16 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Image {
                        img
                    },
                },
            );

            app.add_or_flash(
                (i.to_string() + "_temp:hourly:weather").as_str(),
                UIElementWrapper {
                    position: Point2 { x: x_offset, y: y_offset + 90 },
                    refresh: Default::default(),
                    last_drawn_rect: None,
                    onclick: None,
                    inner: UIElement::Text {
                        text: (temp + "℃").to_string(),
                        scale: 26.0,
                        foreground: color::BLACK,
                        border_px: 0,
                    },
                },
            );
            i += 1;
            x_offset += 80;
        }
    }
}


fn map_icon(icon: String) -> &'static [u8] {
    match icon.as_str() {
        "01n" => N01_ICON,
        "01d" => N01_ICON,
        "02n" => N02_ICON,
        "02d" => N02_ICON,
        "03n" => N03_ICON,
        "03d" => N03_ICON,
        "04n" => N04_ICON,
        "04d" => N04_ICON,
        "09n" => N09_ICON,
        "09d" => N09_ICON,
        "10n" => N10_ICON,
        "10d" => N10_ICON,
        "11n" => N11_ICON,
        "11d" => N11_ICON,
        "13n" => N13_ICON,
        "13d" => N13_ICON,
        "50n" => N50_ICON,
        "50d" => N50_ICON,
        _ => ALERT_ICON
    }
}


fn map_desc(id: u16) -> String {
    let shower_rain = vec![503u16, 504, 522, 531];
    match id {
        200..=233 => "雷雨",
        300..=322 => "细雨",
        i if i == 501 => "中雨",
        i if i == 500 || i == 520 => "小雨",
        i if i == 502 || i == 521 => "大雨",
        i if shower_rain.contains(&i) => "暴雨",
        i if i == 600 || i == 620 => "小雪",
        i if i == 601 || i == 621 => "中雪",
        i if i == 602 || i == 622 => "大雪",
        611..=617 => "雨夹雪",
        701 => "薄雾",
        711 => "烟雾",
        721 => "阴霾",
        i if i == 731 || i == 761 || i == 762 => "灰尘",
        741 => "多雾",
        751 => "风沙",
        771 => "飑",
        781 => "龙卷风",
        800 => "晴天",
        801 => "少云",
        802 => "多云",
        803 => "多云",
        804 => "阴天",
        _ => "未知"
    }.to_string()
}


#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub current: Current,
    pub daily: Vec<Daily>,
    pub hourly: Vec<Hourly>,

}

#[derive(Serialize, Deserialize, Clone)]
pub struct Current {
    pub clouds: u8,
    pub dt: u64,
    pub feels_like: f32,
    pub temp: f32,
    pub weather: Vec<WeatherDesc>,

}

#[derive(Serialize, Deserialize)]
pub struct Daily {
    pub dt: u64,
    pub temp: DailyTemp,
    pub weather: Vec<WeatherDesc>,
}

#[derive(Serialize, Deserialize)]
pub struct Hourly {
    pub dt: u64,
    pub temp: f32,
    pub weather: Vec<WeatherDesc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeatherDesc {
    pub description: String,
    pub icon: String,
    pub id: u16,
    pub main: String,
}

#[derive(Serialize, Deserialize)]
pub struct DailyTemp {
    pub day: f32,
    pub eve: f32,
    pub max: f32,
    pub min: f32,
    pub morn: f32,
    pub night: f32,
}
