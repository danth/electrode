use gtk::prelude::*;
use systemstat::{Platform, platform::PlatformImpl, System};
use crate::{Electrode, make_icon};

pub struct Memory {
    label: gtk::Label,
    system: PlatformImpl
}

impl Electrode for Memory {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        let system = System::new();

        Memory { label, system }
    }

    fn refresh(self) -> Self {
        let memory = self.system.memory().expect("could not measure memory usage");
        let free = (memory.free.as_u64() as f64) / (memory.total.as_u64() as f64);
        let usage = 1.0 - free;
        let percentage = (usage * 100.0).ceil();

        let text = format!("{}%", percentage);
        self.label.set_label(&text);

        self
    }
}
