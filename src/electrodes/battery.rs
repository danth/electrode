use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_label};

pub struct Battery;

impl Electrode for Battery {
    fn setup(parent: &gtk::Box) {
        let percentage_label = make_label(parent);
        let power_label = make_label(parent);

        glib::MainContext::default().spawn_local(clone!(
            @weak percentage_label, @weak power_label =>
            async move {
                loop {
                    match std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                        Ok(percentage) => {
                            let percentage: u64 = percentage
                                .trim()
                                .parse()
                                .expect("parsing battery capacity");

                            let text = format!("{:02.0}%", percentage);
                            percentage_label.set_label(&text);

                            percentage_label.set_visible(true);
                        },
                        Err(_) => {
                            // Most likely there is no battery installed
                            percentage_label.set_visible(false);
                        }
                    }

                    match (
                        std::fs::read_to_string("/sys/class/power_supply/BAT0/voltage_now"),
                        std::fs::read_to_string("/sys/class/power_supply/BAT0/current_now")
                    ) {
                        (Ok(voltage), Ok(current)) => {
                            let voltage: f64 = voltage
                                .trim()
                                .parse()
                                .expect("parsing voltage");
                            let current: f64 = current
                                .trim()
                                .parse()
                                .expect("parsing current");
                            let power = (voltage / 1000000.0) * (current / 1000000.0);

                            let text = format!("{:.1}W", power);
                            power_label.set_label(&text);

                            power_label.set_visible(true);
                        },
                        _ => {
                            // Most likely there is no battery installed
                            power_label.set_visible(false);
                        }
                    }

                    task::sleep(DEFAULT_POLLING_DURATION * 5).await;
                }
            }
        ));
    }
}
