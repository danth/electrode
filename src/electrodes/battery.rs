use async_std::task;
use battery::Manager;
use battery::units::energy::watt_hour;
use battery::units::power::watt;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_label};

fn setup_battery(parent: &gtk::Box, mut battery: battery::Battery) {
    let energy_label = make_label(parent);
    let power_label = make_label(parent);

    let manager = Manager::new().unwrap();

    glib::MainContext::default().spawn_local(clone!(
        @weak energy_label, @weak power_label =>
        async move {
            loop {
                let energy = battery.energy().get::<watt_hour>();
                let energy_text = format!("{:.0}Wh", energy);
                energy_label.set_label(&energy_text);

                let power = battery.energy_rate().get::<watt>();
                let power_text = format!("{:.1}W", power);
                power_label.set_label(&power_text);

                task::sleep(DEFAULT_POLLING_DURATION).await;

                manager.refresh(&mut battery).unwrap();
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

