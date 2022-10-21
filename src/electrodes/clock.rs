use async_std::task;
use chrono::{Local, Datelike, DateTime, Timelike, Utc};
use gtk::prelude::*;
use gtk::glib::{self, clone};
use std::time::Duration;
use crate::electrodes::{Electrode, make_label};

// Sleep until the current time in seconds changes
async fn tick() {
    let now: DateTime<Utc> = Utc::now();

    let duration_since_tick = now - now.with_nanosecond(0).unwrap();

    let mut nanoseconds_until_tick: i64 =
        1000000000 - duration_since_tick.num_nanoseconds().unwrap();

    if nanoseconds_until_tick < 0 {
        // We are in a leap second
        nanoseconds_until_tick = 1000000000 - nanoseconds_until_tick;
    }

    let sleep_duration = Duration::from_nanos(
        nanoseconds_until_tick.try_into().unwrap()
    );

    task::sleep(sleep_duration).await;
}

pub struct Clock;

impl Electrode for Clock {
    fn setup(parent: &gtk::Box) {
        let day_label = make_label(parent);
        let date_label = make_label(parent);
        let time_label = make_label(parent);

        glib::MainContext::default().spawn_local(clone!(
            @weak day_label, @weak date_label, @weak time_label =>
            async move {
                loop {
                    let now: DateTime<Local> = Local::now();

                    day_label.set_label(&now.weekday().to_string());

                    let text = format!("{:04}/{:02}/{:02}", now.year(), now.month(), now.day());
                    date_label.set_label(&text);

                    let text = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
                    time_label.set_label(&text);

                    tick().await;
                }
            }
        ));
    }
}
