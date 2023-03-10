## motion-sensors-system

**WARNING:** this app hasn't been finished - if you have a plan to use please wait 1-3 months till it will be finished.

This is application which uses motion-sensor-pir crate and integrates events from many motion sensors within a house 
and allows react to such events. It sends/receives events from other applications from this system (eg. presence detector) via MQTT.

Python prototype of this app is already used at my family's house as smart alarm with automatic person detection, this is 
rewrite from scratch in Rust due to much better performance. The prototype is running on Raspberry Pi 4 with 4GB RAM and generally does what is suppose to do, but causes significant load on Raspberry.

## Possible use cases
### you can implement simple and complex alarm systems here, for instance with presence-detector app
- *