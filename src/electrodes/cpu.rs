use gtk::prelude::*;
use systemstat::{CPULoad, DelayedMeasurement, Platform, platform::PlatformImpl, System};
use crate::{PollingElectrode, make_icon};

pub struct CPU {
    label: gtk::Label,
    system: PlatformImpl,
    cpu: DelayedMeasurement<CPULoad>
}

impl PollingElectrode for CPU {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        let system = System::new();

        let cpu = system.cpu_load_aggregate()
            .expect("could not prepare CPU load measurement");

        CPU { label, system, cpu }
    }

    fn refresh(&mut self) {
        let cpu = self.cpu.done().expect("could not complete CPU load measurement");
        let usage = 1.0 - cpu.idle;
        let percentage = (usage * 100.0).ceil();

        let text = format!("{}%", percentage);
        self.label.set_label(&text);

        self.cpu = self.system.cpu_load_aggregate()
            .expect("could not prepare CPU load measurement");
    }
}

