use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_icon};

pub struct Cpu;

impl Electrode for Cpu {
    fn setup(parent: &gtk::Box) {
        let (_, label) = make_icon(parent, "");

        glib::MainContext::default().spawn_local(clone!(@weak label => async move {
            let system = System::new();

            let mut cpu = system.cpu_load_aggregate()
                .expect("could not prepare CPU load measurement");

            loop {
                let cpu_done = cpu.done().expect("could not complete CPU load measurement");
                let usage = 1.0 - cpu_done.idle;
                let percentage = (usage * 100.0).ceil();

                let text = format!("{:02}", percentage);
                label.set_label(&text);

                cpu = system.cpu_load_aggregate()
                    .expect("could not prepare CPU load measurement");

                task::sleep(DEFAULT_POLLING_DURATION).await;
            }
        }));
    }
}

