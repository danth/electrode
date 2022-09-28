extern crate async_std;
extern crate chrono;
extern crate gtk;
extern crate gtk_layer_shell;
extern crate libpulse_binding;
extern crate xdg;

mod electrodes;

use gtk::gdk;
use gtk::prelude::*;
use gtk_layer_shell::{Edge, Layer};

use xdg::BaseDirectories;

use std::fs;

use crate::electrodes::Electrode;
use crate::electrodes::clock::Clock;
use crate::electrodes::volume::Volume;
use crate::electrodes::battery::Battery;

fn read_user_css() -> Option<String> {
    BaseDirectories::with_prefix("electrode").unwrap()
        .find_config_file("style.css")
        .map(|config_file| fs::read_to_string(config_file).unwrap())
}

fn load_css(css: &str, priority: u32) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css.as_bytes()).expect("loading CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("could not get default screen"),
        &provider,
        priority
    );
}

fn main() {
    gtk::init().expect("could not initialise GTK");

    load_css(include_str!("style.css"), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    if let Some(user_css) = read_user_css() {
        load_css(&user_css, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_type_hint(gdk::WindowTypeHint::Dock);

    let monitor = gdk::Display::default()
        .expect("could not get default display")
        .monitor(0)
        .expect("could not get first monitor");

    let height = monitor.geometry().height();
    window.set_default_size(33, height);
    window.set_size_request(33, height);

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
    Battery::setup(&statistics_box);

    window.show_all();

    loop {
        gtk::main_iteration_do(true); // Blocking
    }
}
