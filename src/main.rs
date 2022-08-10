extern crate async_std;
extern crate chrono;
extern crate gtk;
extern crate gtk_layer_shell;
extern crate libpulse_binding;
extern crate systemstat;

mod electrodes;

use gtk::gdk;
use gtk::prelude::*;
use gtk_layer_shell::{Edge, Layer};

use crate::electrodes::Electrode;
use crate::electrodes::clock::Clock;
use crate::electrodes::volume::Volume;
use crate::electrodes::network::Network;
use crate::electrodes::memory::Memory;
use crate::electrodes::cpu::Cpu;
use crate::electrodes::cpu_temperature::CpuTemperature;
use crate::electrodes::battery::Battery;

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

    Clock::setup(&main_box);

    let statistics_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    statistics_box.set_vexpand(true);
    statistics_box.set_valign(gtk::Align::End);
    main_box.add(&statistics_box);

    Volume::setup(&statistics_box);
    Network::setup(&statistics_box);
    Memory::setup(&statistics_box);

    let cpu_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    cpu_box.style_context().add_class("electrode");
    main_box.add(&cpu_box);
    Cpu::setup(&cpu_box);
    CpuTemperature::setup(&cpu_box);

    Battery::setup(&statistics_box);

    window.show_all();

    loop {
        gtk::main_iteration_do(true); // Blocking
    }
}
