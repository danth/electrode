use async_std::task;
use battery::Manager;
use battery::units::power::watt;
use battery::units::ratio::percent;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_label};

fn setup_battery(parent: &gtk::Box, mut battery: battery::Battery) {
    let percentage_label = make_label(parent);
    let power_label = make_label(parent);

    glib::MainContext::default().spawn_local(clone!(
        @weak percentage_label, @weak power_label =>
        async move {
            loop {
                let percentage = battery.state_of_charge().get::<percent>();
                let percentage_text = format!("{:.0}%", percentage);
                percentage_label.set_label(&percentage_text);

                let power = battery.energy_rate().get::<watt>();
                let power_text = format!("{:.1}W", power);
                power_label.set_label(&power_text);

                task::sleep(DEFAULT_POLLING_DURATION).await;

                Manager::new().unwrap().refresh(&mut battery).unwrap();
            }
        }
    ));
}

pub struct Battery;

impl Electrode for Battery {
    fn setup(parent: &gtk::Box) {
        for battery in Manager::new().unwrap().batteries().unwrap() {
            setup_battery(parent, battery.unwrap());
        }
    }
}

