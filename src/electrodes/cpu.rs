use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use std::time::Duration;
use crate::electrodes::{Electrode, make_icon};

pub struct CPU;

impl Electrode for CPU {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        glib::MainContext::default().spawn_local(clone!(@weak label => async move {
            let system = System::new();

            let mut cpu = system.cpu_load_aggregate()
                .expect("could not prepare CPU load measurement");

            loop {
                let cpu_done = cpu.done().expect("could not complete CPU load measurement");
                let usage = 1.0 - cpu_done.idle;
                let percentage = (usage * 100.0).ceil();

                let text = format!("{}%", percentage);
                label.set_label(&text);

                cpu = system.cpu_load_aggregate()
                    .expect("could not prepare CPU load measurement");

                task::sleep(Duration::from_secs(1)).await;
            }
        }));
    }
}

