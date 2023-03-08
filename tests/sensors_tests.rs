use motion_sensors_system::config::settings::ConfigStruct;
use motion_sensors_system::states::mode::Mode;

//
//
//#[cfg(test)]
mod tests {
    use motion_sensors_system::{
        config::settings::{read_config, ConfigStruct, MOTION_SENSORS_CHANNEL_DEPTH},
        logic::main_loop::{main_loop, TestData},
        states::mode::Mode,
    };
    use pir_motion_sensor::sensor::config::SensorConfig;
    use pir_motion_sensor::sensor::motion::MotionSensor;
    use std::{
        sync::mpsc::{self, Receiver, SyncSender},
        time::SystemTime,
    };

    #[test]
    fn simple_detections() {
        let configuration: ConfigStruct = read_config(None);
        #[allow(clippy::type_complexity)]
        let (channel_to_send, channel_to_receive): (
            SyncSender<(String, SystemTime)>,
            Receiver<(String, SystemTime)>,
        ) = mpsc::sync_channel(MOTION_SENSORS_CHANNEL_DEPTH);

        let test_cases: Vec<TestData> = vec![
            // detection happens at 1 sec
            TestData {
                time: (1),
                mode: Mode::Automatic,
                sensor: String::from("Sensor1"),
                detections: 2,
            },
            // this detection happens at 5 sec
            TestData {
                time: (5),
                mode: Mode::Automatic,
                sensor: String::from("Sensor1"),
                detections: 4,
            },
            // this detection happens at 10 sec
            TestData {
                time: (10),
                mode: Mode::Automatic,
                sensor: String::from("Sensor3"),
                detections: 10,
            },
        ];

        main_loop(
            configuration,
            channel_to_receive,
            test_cases,
            Some(channel_to_send),
        );
    }
}
