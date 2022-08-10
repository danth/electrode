use gtk::prelude::*;
use systemstat::{Platform, platform::PlatformImpl, System};
use crate::{PollingElectrode, make_icon};

pub struct CPUTemperature {
    label: gtk::Label,
    system: PlatformImpl
}

impl PollingElectrode for CPUTemperature {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        let system = System::new();

        CPUTemperature { label, system }
    }

    fn refresh(&mut self) {
        let cpu_temperature = self.system.cpu_temp()
            .expect("could not measure CPU temperature");

        let text = format!("{}°C", cpu_temperature);
        self.label.set_label(&text);
    }
}
