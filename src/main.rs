use chrono::DateTime;
use chrono::Local;
use env_logger::Builder;
use log::{info, LevelFilter};
use motion_sensors_system::config::settings::{
    read_config, ConfigStruct, MOTION_SENSORS_CHANNEL_DEPTH,
};
use pir_motion_sensor::sensor::helpers::spawn_detection_threads;
use tokio_util::sync::CancellationToken;
use pir_motion_sensor::sensor::motion::MotionSensor;
// use soloud::*;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};

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

    loop {
        info!("starting main...");

        let sensors_inititialized = false;
        // channel for sensor data
        #[allow(clippy::type_complexity)]
        let (detections_channel_sender, mut detections_channel_receiver): (
            Sender<(String, SystemTime)>,
            Receiver<(String, SystemTime)>,
        ) = mpsc::channel(MOTION_SENSORS_CHANNEL_DEPTH);

        info!("communication channel created");

        let configuration: ConfigStruct = read_config(None);

        info!("starting program...");

        if sensors_inititialized {
            info!("this is not first reload");
        }

        let mut sensors = Vec::new();

        configuration
            .clone()
            .motion_sensors_list
            .into_iter()
            .for_each(|sensor| {
                let s = MotionSensor::new(
                    sensor.name,
                    sensor.pin_number,
                    sensor.refresh_rate_milisecs,
                    sensor.motion_time_period_milisecs,
                    sensor.minimal_triggering_number,
                    detections_channel_sender.clone(),
                    None,
                );

                sensors.push(Arc::new(Mutex::new(s)))
            });

        // cancellation token which can be later used to stop sensors threads
        let token = CancellationToken::new();

        // helper function to run important threads (via tokio::spawn)
        // you don't have deal this is you don't want to - just leave it as it is
        spawn_detection_threads(sensors, token.clone());

        //
        // main loop: here we put logic to handle valid detections, place your code here
        //
        loop {
            if let Ok(detection_message) = detections_channel_receiver.try_recv() {
                // valid detection received
                // each "valid" detection contains the sensor name and time of detection as SystemTime
                let (detection_name, detection_time) = detection_message;

                let human_time_pre: DateTime<Local> = detection_time.into();
                let human_time_final = human_time_pre.format("[%Y-%m-%d %H:%M:%S]");

                println!("{human_time_final}: {detection_name}");
                //
                // put your action here like alarm or turn on/off light
                // to interact with rest GPIOs you can check rppal lib examples here: https://github.com/golemparts/rppal/tree/master/examples
                //
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        // end outer loop
    }
}