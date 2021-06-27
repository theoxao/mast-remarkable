pub static CLOCK_MINUTE: &str = "clock_minute";
pub static CLOCK_HOUR: &str = "clock_hour";
pub static CLOCK_DATE: &str = "clock_date";
pub static CLOCK_WEEK: &str = "clock_week";
pub static LUNI_DATE: &str = "luni_date";
pub static BATTERY_STATE_LABEL:&str = "battery_state";
pub static BATTER_DISPLAY_LABEL:&str =  "batter_display";

pub(crate) static API_HOST: &'static str = "http://api.theoxao.com";
pub(crate) static WIFI_ON_ICON: &[u8] = include_bytes!("../assets/icon/wifi_on.png") as &[u8];
pub(crate) static WIFI_OFF_ICON: &[u8] = include_bytes!("../assets/icon/wifi_off.png") as &[u8];
pub(crate) static WIFI_CONNECTED_ICON: &[u8] = include_bytes!("../assets/icon/wifi_connected.png") as &[u8];
pub(crate) static CHARGING_ICON :&[u8] = include_bytes!("../assets/icon/charging.png") as &[u8];

pub(crate) static SWITCH_ON_ICON: &[u8] = include_bytes!("../assets/icon/switch_on.png") as &[u8];
pub(crate) static SWITCH_OFF_ICON: &[u8] = include_bytes!("../assets/icon/switch_off.png") as &[u8];


#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonResponse<T> {
    pub status: u8,
    pub error: Option<String>,
    pub data: Option<T>,
}