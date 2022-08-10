extern crate chrono;
extern crate gtk;
extern crate gtk_layer_shell;
extern crate pulsectl;
extern crate systemstat;

mod electrodes;

use chrono::{Local, DateTime, Timelike};
use gtk::gdk;
use gtk::prelude::*;
use gtk_layer_shell::{Edge, Layer};
use std::thread;
use std::time::Duration;

use crate::electrodes::clock::Clock;
use crate::electrodes::volume::Volume;
use crate::electrodes::network::Network;
use crate::electrodes::memory::Memory;
use crate::electrodes::cpu::Cpu;
use crate::electrodes::cpu_temperature::CpuTemperature;
use crate::electrodes::battery::Battery;

pub trait Electrode {
    fn initialize(parent: &gtk::Box) -> Self;
    fn refresh(&mut self);
}

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

pub fn make_icon(parent_box: &gtk::Box, icon: &str) -> (gtk::Box, gtk::Label) {
    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 3);
    parent_box.add(&box_);

    let icon = gtk::Label::new(Some(icon));
    icon.style_context().add_class("icon");
    box_.add(&icon);

    let label = gtk::Label::new(None);
    label.set_justify(gtk::Justification::Center);
    box_.add(&label);

    (box_, label)
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
    gtk::init().expect("could not initialise GTK");

    load_css();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

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

    let mut clock = Clock::initialize(&main_box);

    let statistics_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    statistics_box.set_vexpand(true);
    statistics_box.set_valign(gtk::Align::End);
    main_box.add(&statistics_box);

    let mut volume = Volume::initialize(&statistics_box);
    let mut network = Network::initialize(&statistics_box);
    let mut memory = Memory::initialize(&statistics_box);
    let mut cpu = Cpu::initialize(&statistics_box);
    let mut cpu_temperature = CpuTemperature::initialize(&statistics_box);
    let mut battery = Battery::initialize(&statistics_box);

    window.show_all();

    loop {
        clock.refresh();
        volume.refresh();
        network.refresh();
        memory.refresh();
        cpu.refresh();
        cpu_temperature.refresh();
        battery.refresh();

        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }

        tick();
    }
}
