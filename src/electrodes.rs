pub mod clock;
pub mod volume;
pub mod battery;

use gtk::prelude::*;
use std::time::Duration;

pub const DEFAULT_POLLING_DURATION: Duration = Duration::from_millis(2500);

pub fn make_label(parent_box: &gtk::Box) -> gtk::Label {
    let label = gtk::Label::new(None);
    label.set_justify(gtk::Justification::Center);

    parent_box.add(&label);

    label
}

pub trait Electrode {
    fn setup(parent: &gtk::Box);
}
