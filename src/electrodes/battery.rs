use gtk::prelude::*;
use systemstat::{Platform, platform::PlatformImpl, System};
use crate::{Electrode, make_icon};

pub struct Battery {
    box_: gtk::Box,
    label: gtk::Label,
    system: PlatformImpl
}

impl Electrode for Battery {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        let system = System::new();

        Battery { box_, label, system }
    }

    fn refresh(&mut self) {
        match self.system.battery_life() {
            Ok(battery) => {
                let percentage = (battery.remaining_capacity * 100.0).ceil();

                let text = format!("{}%", percentage);
                self.label.set_label(&text);

                self.box_.set_visible(true);
            },

            Err(_) => {
                // Most likely there is no battery installed
                self.box_.set_visible(false);
            }
        }
    }
}
