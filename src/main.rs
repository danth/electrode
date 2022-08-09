extern crate chrono;
use chrono::{Local, Datelike, DateTime, Timelike};

extern crate gtk;
use gtk::prelude::*;
use gtk::gdk;
use gtk::glib::{self, clone};

extern crate gtk_layer_shell;
use gtk_layer_shell::{Edge, Layer};

extern crate systemstat;
use systemstat::{Platform, System};

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
fn start_tick_loop(day_label: &gtk::Label, date_label: &gtk::Label, time_label: &gtk::Label) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        loop {
            let now: DateTime<Local> = Local::now();
            sender.send(now).expect("could not send through channel");

            tick();
        }
    });

    receiver.attach(
        None,
        clone!(
            @weak day_label, @weak date_label, @weak time_label =>
            @default-return Continue(false),
            move |now| {
                let text = format!("{}", now.weekday());
                day_label.set_label(&text);

                let text = format!("{}\n{:02}\n{:02}", now.year(), now.month(), now.day());
                date_label.set_label(&text);

                let text = format!("{:02}\n{:02}\n{:02}", now.hour(), now.minute(), now.second());
                time_label.set_label(&text);

                Continue(true)
            }
        )
    );
}

fn start_cpu_loop(label: &gtk::Label) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let system = System::new();

        loop {
            let cpu = system.cpu_load_aggregate().expect("could not prepare CPU load measurement");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().expect("could not complete CPU load measurement");

            let usage = 1.0 - cpu.idle;

            sender.send(usage).expect("could not send through channel");
        }
    });

    receiver.attach(
        None,
        clone!(
            @weak label =>
            @default-return Continue(false),
            move |usage| {
                let percentage = (usage * 100.0).ceil();
                let text = format!("{}%", percentage);
                label.set_label(&text);

                Continue(true)
            }
        )
    );
}

fn start_battery_loop(box_: &gtk::Box, label: &gtk::Label) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let system = System::new();

        loop {
            match system.battery_life() {
                Ok(battery) => {
                    sender.send(Some(battery.remaining_capacity))
                        .expect("could not send through channel");
                },
                Err(_) => {
                    // Most likely there is no battery installed
                    sender.send(None).expect("could not send through channel");
                }
            }

            thread::sleep(Duration::from_secs(10));
        }
    });

    receiver.attach(
        None,
        clone!(
            @weak box_, @weak label =>
            @default-return Continue(false),
            move |capacity| {
                match capacity {
                    Some(capacity) => {
                        let percentage = (capacity * 100.0).ceil();
                        let text = format!("{}%", percentage);
                        label.set_label(&text);

                        box_.set_visible(true);
                    }
                    None => {
                        box_.set_visible(false);
                    }
                }

                Continue(true)
            }
        )
    );
}

fn make_icon_electrode(main_box: &gtk::Box, icon: &str) -> (gtk::Box, gtk::Label) {
    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 3);
    box_.set_vexpand(true);
    box_.set_valign(gtk::Align::End);
    box_.style_context().add_class("electrode");
    main_box.add(&box_);

    let icon = gtk::Label::new(Some(icon));
    icon.style_context().add_class("icon");
    box_.add(&icon);

    let label = gtk::Label::new(None);
    box_.add(&label);

    (box_, label)
}

fn activate(application: &gtk::Application) {
    load_css();

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

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&main_box);

    let clock_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    clock_box.set_valign(gtk::Align::Start);
    main_box.add(&clock_box);

    let day_label = gtk::Label::new(None);
    day_label.style_context().add_class("electrode");
    clock_box.add(&day_label);

    let date_label = gtk::Label::new(None);
    date_label.set_justify(gtk::Justification::Center);
    date_label.style_context().add_class("electrode");
    clock_box.add(&date_label);

    let time_label = gtk::Label::new(None);
    time_label.set_justify(gtk::Justification::Center);
    time_label.style_context().add_class("electrode");
    clock_box.add(&time_label);

    start_tick_loop(&day_label, &date_label, &time_label);

    let (_, cpu_label) = make_icon_electrode(&main_box, "");
    start_cpu_loop(&cpu_label);

    let (battery_box, battery_label) = make_icon_electrode(&main_box, "");
    start_battery_loop(&battery_box, &battery_label);

    window.show_all();
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_bytes!("style.css")).expect("loading CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("could not get default screen"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.danth.ticker")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(activate);

    app.run();
}
