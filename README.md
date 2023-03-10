## motion-sensors-system

**WARNING: this app hasn't been finished yet - if you have a plan to use it please wait till it's finished - it should take no more than 3 months in the worst case scenario.**

&nbsp;
## What's this

This is an application which uses [motion-sensor-pir](https://github.com/mateusz-szczyrzyca/pir-motion-sensor) crate and integrates events from many motion sensors within a house and allows reacting to such events. 

It sends/receives events from other applications from this system (eg. presence detector) via MQTT.

Python prototype of this app is already used at my family's house as smart alarm with automatic person detection, this is 
rewrite from scratch in Rust due to much better performance and resource usage. The prototype is running on Raspberry Pi 4 with 4GB RAM and generally does what is suppose to do, but causes significant load on Raspberry.
&nbsp;
## TODO
- [ ] MQTT green thread to receive events from other systems/sensors
- [ ] MQTT green thread to send events to other systems/sensors
- [ ] State implementations
- [ ] Live reload config file in reaction to events from MQTT
- [ ] Live reload sensors config in reaction to events from MQTT
- [ ] Examples of simple alarm algorithms
- [ ] More tests

&nbsp;
## Possible use cases

- You can implement simple and complex alarm systems here with many different algorithms, for instance with presence-detector the app prototype allows such alarm system to be fully automatic and requires no supervision from authorized persons, arming/disarming actions are fully automatic and accurate

  Presence detector code also will be published soon.

- Various reactions to motion detection such as turning on/off lights, opening garage doors, turn on/off heating systems, etc. 
  There are no limits in such use cases so if you want to use Rust on your Raspberry this app should be a good choice.