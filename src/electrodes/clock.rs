use chrono::{Local, Datelike, DateTime, Timelike};
use gtk::prelude::*;
use crate::{Electrode, make_icon};

pub struct Clock {
    day_label: gtk::Label,
    date_label: gtk::Label,
    time_label: gtk::Label
}

impl Electrode for Clock {
    fn initialize(parent: &gtk::Box) -> Self {
        let clock_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        clock_box.set_vexpand(true);
        clock_box.set_valign(gtk::Align::Start);
        parent.add(&clock_box);

        let (day_box, day_label) = make_icon(&clock_box, "");
        day_box.style_context().add_class("electrode");

        let (date_box, date_label) = make_icon(&clock_box, "");
        date_box.style_context().add_class("electrode");

        let (time_box, time_label) = make_icon(&clock_box, "");
        time_box.style_context().add_class("electrode");

        Clock { day_label, date_label, time_label }
    }

    fn refresh(&mut self) {
        let now: DateTime<Local> = Local::now();

        let text = format!("{}", now.weekday());
        self.day_label.set_label(&text);

        let text = format!("{}\n{:02}\n{:02}", now.year(), now.month(), now.day());
        self.date_label.set_label(&text);

        let text = format!("{:02}\n{:02}\n{:02}", now.hour(), now.minute(), now.second());
        self.time_label.set_label(&text);
    }
}
