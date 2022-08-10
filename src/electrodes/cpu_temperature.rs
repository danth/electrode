use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use std::time::Duration;
use crate::electrodes::{Electrode, make_icon};

pub struct CPUTemperature;

impl Electrode for CPUTemperature {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        glib::MainContext::default().spawn_local(clone!(@weak label => async move {
            let system = System::new();

            loop {
                let cpu_temperature = system.cpu_temp()
                    .expect("could not measure CPU temperature");

                let text = format!("{}°C", cpu_temperature);
                label.set_label(&text);

                task::sleep(Duration::from_secs(1)).await;
            }
        }));
    }
}
