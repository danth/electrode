use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use std::time::Duration;
use crate::electrodes::{Electrode, make_icon};

pub struct Memory;

impl Electrode for Memory {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        glib::MainContext::default().spawn_local(clone!(@weak label => async move {
            let system = System::new();

            loop {
                let memory = system.memory().expect("could not measure memory usage");
                let free = (memory.free.as_u64() as f64) / (memory.total.as_u64() as f64);
                let usage = 1.0 - free;
                let percentage = (usage * 100.0).ceil();

                let text = format!("{}%", percentage);
                label.set_label(&text);

                task::sleep(Duration::from_secs(1)).await;
            }
        }));
    }
}
