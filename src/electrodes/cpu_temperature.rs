use gtk::prelude::*;
use systemstat::{Platform, platform::PlatformImpl, System};
use crate::{Electrode, make_icon};

pub struct CpuTemperature {
    label: gtk::Label,
    system: PlatformImpl
}

impl Electrode for CpuTemperature {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(parent, "");
        box_.style_context().add_class("electrode");

        let system = System::new();

        CpuTemperature { label, system }
    }

    fn refresh(&mut self) {
        let cpu_temperature = self.system.cpu_temp()
            .expect("could not measure CPU temperature");

        let text = format!("{}°C", cpu_temperature.ceil());
        self.label.set_label(&text);
    }
}
