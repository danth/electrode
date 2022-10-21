use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_label};

pub struct Battery;

impl Electrode for Battery {
    fn setup(parent: &gtk::Box) {
        let label = make_label(parent);

        glib::MainContext::default().spawn_local(clone!(
            @weak label =>
            async move {
                loop {
                    match std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                        Ok(percentage) => {
                            let percentage: u64 = percentage
                                .trim()
                                .parse()
                                .expect("parsing battery capacity");

                            let text = format!("{:02.0}%", percentage);
                            label.set_label(&text);

                            label.set_visible(true);
                        },

                        Err(_) => {
                            // Most likely there is no battery installed
                            label.set_visible(false);
                        }
                    }

                    task::sleep(DEFAULT_POLLING_DURATION * 5).await;
                }
            }
        ));
    }
}
