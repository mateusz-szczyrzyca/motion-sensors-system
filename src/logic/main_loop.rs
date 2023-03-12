use crate::config::settings::{read_config, ConfigStruct, STOP_LOOP_COUNT_MAX};
use crate::states::mode::Mode;
use ahash::AHashMap;
use log::{info, warn};
use soloud::{audio, AudioExt, LoadExt, Soloud};
use std::path::Path;
use std::sync::mpsc::SyncSender;
use std::time::{Duration, Instant};
use std::{fs, sync::mpsc::Receiver, thread, time::SystemTime};

#[derive(Debug, Clone)]
pub struct TestData {
    pub time: u64,
    pub mode: Mode,
    pub sensor: String,
    pub detections: u64,
}

pub fn main_loop(
    config: ConfigStruct,
    channel_to_receive: Receiver<(String, SystemTime)>,
    testing_data: Vec<TestData>,
    testing_send_channel: Option<SyncSender<(String, SystemTime)>>,
) {
    let mut testing_mode: bool = false;
    let mut testing_scenarios_count = 0;

    // if this is testing mode, then detections are "simulated" and sent through the channel.
    // in that case we don't need data from gpio as we simulate this
    if !testing_data.is_empty() {
        testing_mode = true;
        testing_scenarios_count = testing_data.len();
    }

    info!("starting...");

    loop {
        info!("reading configuration");
        let mut configuration = config.clone();

        info!("main loop started.");

        let mut stop_loop: bool = false;
        let mut stop_loop_count = 0;
        let mut stop_loop_count_max = STOP_LOOP_COUNT_MAX;
        let mut detection: bool;
        let mut no_detection: bool;

        let mut testing_index = 0;
        let testing_time_start = Instant::now();

        let mut sensor_name: String = String::from("");
        let mut sensor_detection_time: SystemTime = SystemTime::now();

        let mut last_log_time = Instant::now();
        let mut last_detection_time = Instant::now();
        let mut last_config_load = Instant::now();
        let mut last_report_time = Instant::now();

        let mut detector_time_name_map: AHashMap<SystemTime, String> = AHashMap::new();
        let mut detector_times_list: Vec<SystemTime> = Vec::new();
        let mut detector_report_count: AHashMap<String, u32> = AHashMap::new();

        // let sl = Soloud::default().unwrap();
        // let mut end_tone = audio::Wav::default();
        // end_tone
        //     .load(&std::path::Path::new("/home/pi/wav/short-click.wav"))
        //     .unwrap();

        loop {
            no_detection = true;
            detection = false;

            // unit testing mode
            if testing_mode && !stop_loop {
                let current_test_detection = &testing_data[testing_index];

                if testing_time_start.elapsed().as_secs() >= current_test_detection.time {
                    println!("test case: {:?}", current_test_detection.clone());
                    println!("scenarios_count: {}", testing_scenarios_count);
                    println!("testing_index: {}", testing_index);
                    testing_index += 1;

                    sensor_name = current_test_detection.sensor.clone();
                    sensor_detection_time = SystemTime::now();
                    let channel = testing_send_channel.as_ref().unwrap().clone();

                    for _ in 0..current_test_detection.detections {
                        channel
                            .send((sensor_name.clone(), sensor_detection_time))
                            .unwrap();
                    }

                    if testing_index >= testing_scenarios_count {
                        stop_loop_count_max = current_test_detection.detections;
                        stop_loop = true;
                    }
                }
            }

            // receiving real detections
            if let Ok(sensor_data) = channel_to_receive.try_recv() {
                detection = true;
                (sensor_name, sensor_detection_time) = sensor_data;
            };

            // vars initialized when detection happens
            if detection {
                no_detection = false;
                last_detection_time = Instant::now();
            }

            // BEGIN: reporting instead of
            // Possible func: LogReportingDetections(Config.detection_report, these vec and maps and times (4))
            if configuration.detection_report {
                // TODO: And NotInTestingState
                if last_report_time.elapsed().as_secs()
                    >= configuration.detection_report_period_secs
                {
                    detector_times_list.sort();

                    for time in &detector_times_list {
                        let detector_name = detector_time_name_map.get(time).unwrap().to_string();
                        *detector_report_count.entry(detector_name).or_insert(1) += 1
                    }

                    let mut count = 1;
                    for (detector_name, detection_count) in detector_report_count.iter() {
                        info!(
                            "[{}] detection report: {}, count: {}",
                            count, detector_name, detection_count
                        );
                        count += 1;
                    }

                    last_report_time = Instant::now();
                    detector_times_list.clear();
                    detector_time_name_map.clear();
                    detector_report_count.clear();
                }

                if detection {
                    detector_times_list.push(sensor_detection_time);
                    detector_time_name_map.insert(sensor_detection_time, sensor_name.clone());
                }
            }

            if !configuration.detection_report {
                // Possible func: LogInstantDetections(detection, sensor_name, sensor_time)

                if detection {
                    // sl.play(&end_tone);
                    info!(
                        "Detection, sensor: {}, time: {:?}",
                        sensor_name, sensor_detection_time
                    );
                }
            }

            if no_detection {
                let last_detection_time_secs = last_detection_time.elapsed().as_secs();
                if last_detection_time_secs >= configuration.detection_report_period_secs
                    // && last_detection_time_secs % configuration.detection_report_period_secs == 0
                    && last_log_time.elapsed().as_secs()
                        >= configuration.no_detection_report_period_secs
                {
                    last_log_time = Instant::now();
                    let last_detection_time_mins = last_detection_time_secs / 60;
                    let last_any_detection =
                        warn!("no detection in the last {last_detection_time_mins} mins");
                }
            }

            if last_config_load.elapsed().as_secs() >= configuration.config_reload_time_secs {
                if configuration.config_reload_notify {
                    info!("re-read config file");
                }

                let reload_file =
                    format!("{}/{}", &configuration.basedir, &configuration.reload_file);
                if Path::new(&reload_file).is_file() {
                    warn!("state file present - reloading");
                    // remove file
                    fs::remove_file(&reload_file).unwrap_or(());

                    // going to outer loop to reinit everything
                    info!("reloading sensors");
                    stop_loop = true;
                    break;
                }
                configuration = read_config(Some(configuration));
                last_config_load = Instant::now();
            }

            if stop_loop {
                if stop_loop_count >= stop_loop_count_max {
                    break;
                }
                stop_loop_count += 1;
            }

            thread::sleep(Duration::from_millis(configuration.main_loop_time_milisecs));
        }

        if stop_loop {
            break;
        }
        // end outer loop
    }
}
