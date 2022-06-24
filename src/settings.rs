use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "ServiceName")]
    pub service_name: String,

    #[serde(rename = "TickersToSubscribe")]
    pub tickers_to_subscribe: Vec<String>
}