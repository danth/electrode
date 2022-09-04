use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_icon};

pub struct Battery;

impl Electrode for Battery {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(parent, "");

        glib::MainContext::default().spawn_local(clone!(
            @weak box_, @weak label =>
            async move {
                let system = System::new();

                loop {
                    match system.battery_life() {
                        Ok(battery) => {
                            let percentage = battery.remaining_capacity * 100.0;

                            let text = format!("{:02.0}", percentage);
                            label.set_label(&text);

                            box_.set_visible(true);
                        },

                        Err(_) => {
                            // Most likely there is no battery installed
                            box_.set_visible(false);
                        }
                    }

                    task::sleep(DEFAULT_POLLING_DURATION * 5).await;
                }
            }
        ));
    }
}
