// use std::process;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
pub struct Timer {
    start: Arc<Mutex<Duration>>,
    control: Arc<Condvar>,
    paused: Arc<Mutex<bool>>,
}

impl Timer {
    pub fn new(secs: u64) -> Self {
        Self {
            start: Arc::new(Mutex::new(Duration::new(secs, 0))),
            control: Arc::new(Condvar::new()),
            paused: Arc::new(Mutex::new(false)),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.start.lock().unwrap().is_zero()
    }

    pub fn remaining_duration(&self) -> Duration {
        self.start.lock().unwrap().clone()
    }

    pub fn is_running(&self) -> bool {
        !self.is_finished() && !*self.paused.lock().unwrap()
    }

    pub fn countdown_start(&self) -> JoinHandle<()> {
        // let secs: u64 = *self.start.lock().unwrap();
        let duration: Arc<Mutex<Duration>> = Arc::clone(&self.start);
        let paused: Arc<Mutex<bool>> = Arc::clone(&self.paused);
        let control: Arc<Condvar> = Arc::clone(&self.control);
        thread::spawn(move || Timer::countdown(duration, paused, control))
    }

    pub fn pause(&self) {
        if self.is_finished() {
            return;
        }
        let mut paused: std::sync::MutexGuard<'_, bool> = self.paused.lock().unwrap();
        *paused = true;
        drop(paused);
    }

    pub fn resume(&self) {
        if self.is_finished() {
            return;
        }
        let mut paused: std::sync::MutexGuard<'_, bool> = self.paused.lock().unwrap();
        *paused = false;
        drop(paused);
        self.control.notify_one();
    }

    fn countdown(duration: Arc<Mutex<Duration>>, paused: Arc<Mutex<bool>>, control: Arc<Condvar>) {
        const PRECISION: Duration = Duration::from_millis(20);
        let control: &Condvar = &*control;
        let mut last_frame_time: Instant = Instant::now();
        loop {
            let mut paused: std::sync::MutexGuard<'_, bool> = paused.lock().unwrap();
            while *paused {
                paused = control.wait(paused).unwrap();
                last_frame_time = Instant::now();
            }
            let current_time = Instant::now();
            let mut duration = duration.lock().unwrap();
            if current_time - last_frame_time >= PRECISION {
                *duration = duration.saturating_sub(current_time - last_frame_time);
                last_frame_time = current_time;
            }
            if duration.is_zero() {
                break;
            }
        }
    }
}
