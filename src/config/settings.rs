use std::fs;

use serde::Deserialize;

use pir_motion_sensor::sensor::config::SensorConfig;

pub const CONFIG_FILENAME: &str = "config.toml";
pub const STOP_LOOP_COUNT_MAX: u64 = 10;
pub const MOTION_SENSORS_CHANNEL_DEPTH: usize = 10;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigStruct {
    pub main_loop_time_milisecs: u64,
    pub motion_sensors_list: Vec<SensorConfig>,
    pub config_reload_time_secs: u64,
    pub config_reload_notify: bool,
    pub detection_report: bool,
    pub detection_report_period_secs: u64,
    pub no_detection_report_period_secs: u64,
    pub basedir: String,
    pub reload_file: String,
}

pub fn read_config(cfg: Option<ConfigStruct>) -> ConfigStruct {
    if cfg.is_none() {
        let config_data = fs::read_to_string(CONFIG_FILENAME).expect("Cannot read config file {}");
        let config_struct = toml::from_str(config_data.as_str()).unwrap();
        return config_struct;
    }
    cfg.unwrap()
}
