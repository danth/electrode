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

    let width = monitor.geometry().width();
    window.set_default_size(width, 25);
    window.set_size_request(width, 25);

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_monitor(&window, &monitor);
    gtk_layer_shell::set_layer(&window, Layer::Bottom);
    gtk_layer_shell::set_anchor(&window, Edge::Top, true);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);
    gtk_layer_shell::set_keyboard_interactivity(&window, false);

    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    main_box.set_homogeneous(true);
    window.add(&main_box);

    let left_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    left_box.set_halign(gtk::Align::Start);
    main_box.add(&left_box);

    Battery::setup(&left_box);

    let center_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    center_box.set_halign(gtk::Align::Center);
    main_box.add(&center_box);

    Clock::setup(&center_box);

    let right_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    right_box.set_halign(gtk::Align::End);
    main_box.add(&right_box);

    Volume::setup(&right_box);

    window.show_all();

    loop {
        gtk::main_iteration_do(true); // Blocking
    }
}
