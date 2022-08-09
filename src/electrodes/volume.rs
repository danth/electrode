use gtk::prelude::*;
use pulsectl::controllers::{DeviceControl, SinkController};
use crate::{Electrode, make_icon};

pub struct Volume { label: gtk::Label }

impl Electrode for Volume {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");
        Volume { label }
    }

    fn refresh(self) -> Self {
        let mut handler = SinkController::create().expect("could not connect to PulseAudio");
        let device = handler.get_default_device().expect("could not get default PulseAudio device");

        if device.mute {
            self.label.set_label("Mute");
        } else {
            let text = format!("{}%", device.volume.avg());
            self.label.set_label(&text);
        }

        self
    }
}
