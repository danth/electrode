extern crate chrono;
use chrono::{Local, Datelike, DateTime, Timelike};

extern crate gtk;
use gtk::prelude::*;
use gtk::gdk;
use gtk::glib::{self, clone};

extern crate gtk_layer_shell;
use gtk_layer_shell::{Edge, Layer};

use std::thread;
use std::time::Duration;

// Sleep until the current time in seconds changes
fn tick() {
    let now: DateTime<Local> = Local::now();

    let duration_since_tick = now - now.with_nanosecond(0).unwrap();

    let mut nanoseconds_until_tick: i64 =
        1000000000 - duration_since_tick.num_nanoseconds().unwrap();

    if nanoseconds_until_tick < 0 {
        // We are in a leap second
        nanoseconds_until_tick = 1000000000 - nanoseconds_until_tick;
    }

    let sleep_duration = Duration::from_nanos(
        nanoseconds_until_tick.try_into().unwrap()
    );

    thread::sleep(sleep_duration);
}

// Set the given label to the current time whenever it changes
fn start_tick_loop(label: &gtk::Label) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        loop {
            let now: DateTime<Local> = Local::now();
            let text = format!(
                "{}\n{}\n{:02}\n{:02}\n{:02}\n{:02}\n{:02}",
                now.weekday(),
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );

            sender.send(text).expect("could not send through channel");

            tick();
        }
    });

    receiver.attach(
        None,
        clone!(
            @weak label =>
            @default-return Continue(false),
            move |text| {
                label.set_label(&text);
                Continue(true)
            }
        )
    );
}

fn activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .build();

    window.set_type_hint(gdk::WindowTypeHint::Dock);

    let monitor = gdk::Display::default()
        .expect("could not get default display")
        .monitor(0)
        .expect("could not get first monitor");

    let height = monitor.geometry().height();
    window.set_default_size(40, height);
    window.set_size_request(40, height);

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_monitor(&window, &monitor);
    gtk_layer_shell::set_layer(&window, Layer::Bottom);
    gtk_layer_shell::set_anchor(&window, Edge::Left, true);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);
    gtk_layer_shell::set_keyboard_interactivity(&window, false);

    let label = gtk::Label::new(None);
    window.add(&label);
    start_tick_loop(&label);

    window.show_all();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.danth.ticker")
        .build();
    app.connect_activate(activate);
    app.run();
}
