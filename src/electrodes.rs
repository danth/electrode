pub mod clock;
pub mod volume;
pub mod network;
pub mod memory;
pub mod cpu;
pub mod cpu_temperature;
pub mod battery;

use gtk::prelude::*;
use std::time::Duration;

pub const DEFAULT_POLLING_DURATION: Duration = Duration::from_millis(2500);

pub fn make_icon(parent_box: &gtk::Box, icon: &str) -> (gtk::Box, gtk::Label) {
    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 3);
    box_.style_context().add_class("electrode");
    parent_box.add(&box_);

    let icon = gtk::Label::new(Some(icon));
    icon.style_context().add_class("icon");
    box_.add(&icon);

    let label = gtk::Label::new(None);
    label.set_justify(gtk::Justification::Center);
    box_.add(&label);

    (box_, label)
}

pub trait Electrode {
    fn setup(parent: &gtk::Box);
}
