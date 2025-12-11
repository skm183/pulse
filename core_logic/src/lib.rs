use std::{thread, time};
use std::sync::mpsc::Sender;
use std::time::SystemTime;

pub enum Event {
    TaskStart {
        name: String,
        end_time: SystemTime
    },
    TaskEnd {
        name: String
    }
}

pub struct Timer {
    eye_interval: u64,
    water_interval: u64,
    tx: Sender<Event>
}

impl Timer {
    pub fn new(eye: u64, water:u64, tx: Sender<Event>) -> Self {
        Self { eye_interval: eye, water_interval: water, tx }
    }

    pub fn start(&self) {
        let tx_eye = self.tx.clone();
        let eye_min = self.eye_interval;

        let tx_water = self.tx.clone();
        let water_min = self.water_interval;

        thread::spawn(move || {
            loop{
                let now = SystemTime::now();
                let duration = time::Duration::from_secs(eye_min*60);
                let end_time = now+duration;

                let _ = tx_eye.send(Event::TaskStart { name: "Eyes".to_string(), end_time });
                thread::sleep(duration);
                let _ = tx_eye.send(Event::TaskEnd { name: "Eyes".to_string() });
            }
        });

        thread::spawn(move || {
            loop{
                let now = SystemTime::now();
                let duration = time::Duration::from_secs(water_min*60);
                let end_time = now+duration;

                let _ = tx_water.send(Event::TaskStart { name: "Water".to_string(), end_time });
                thread::sleep(duration);
                let _ = tx_water.send(Event::TaskEnd { name: "Water".to_string() });
            }
        }); 
    }

}