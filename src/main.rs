use chrono::Local;
use env_logger::Builder;
use log::{info, warn, LevelFilter};
use motion_sensors_system::config::settings::{
    read_config, ConfigStruct, MOTION_SENSORS_CHANNEL_DEPTH,
};
use motion_sensors_system::logic::main_loop::main_loop;
use pir_motion_sensor::sensor::motion::MotionSensor;
// use soloud::*;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    time::SystemTime,
};

mod config;
mod logic;
mod states;

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let mut stop_commands_list_channels: Vec<Sender<bool>> = Vec::new();
    let mut sensors_list = State::default();

    loop {
        info!("starting main...");

        let sensors_inititialized = false;
        #[allow(clippy::type_complexity)]
        let (channel_to_send, channel_to_receive): (
            SyncSender<(String, SystemTime)>,
            Receiver<(String, SystemTime)>,
        ) = mpsc::sync_channel(MOTION_SENSORS_CHANNEL_DEPTH);

        info!("communication channel created");

        let configuration: ConfigStruct = read_config(None);

        info!("starting program...");

        if sensors_inititialized {
            info!("this is not first reload");
        }

        configuration
            .clone()
            .motion_sensors_list
            .into_iter()
            .for_each(|sensor| {
                let mut s = MotionSensor::new(
                    sensor.name,
                    sensor.pin_number,
                    sensor.refresh_rate_milisecs,
                    sensor.motion_time_period_milisecs,
                    sensor.minimal_triggering_number,
                    channel_to_send.clone(),
                    None,
                );
                sensors_list.add_to_list(s.clone());

                let (sender, receiver) = mpsc::channel();

                tokio::task::spawn_blocking(move || {
                    s.start_detector(receiver);
                });
                stop_commands_list_channels.push(sender);
            });

        let test_data = Vec::new();

        // long running main loop: receive detections from sensors via channel and process them accordingly
        // if the loops stops then we "reload" sensors and start it again
        main_loop(configuration.clone(), channel_to_receive, test_data, None);

        for c in &stop_commands_list_channels {
            warn!("stop commands sending...");
            c.send(true).unwrap();
        }

        // end outer loop
    }
}

pub struct State {
    list_sensors: Vec<MotionSensor>,
}

impl Default for State {
    fn default() -> Self {
        let list_sensors: Vec<MotionSensor> = Vec::new();
        Self { list_sensors }
    }
}

impl State {
    fn add_to_list(&mut self, sensor: MotionSensor) {
        self.list_sensors.push(sensor);
    }
}
