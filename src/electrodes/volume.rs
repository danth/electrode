use gtk::prelude::*;
use gtk::glib::{self, clone};
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::context::introspect::{ServerInfo, SinkInfo};
use libpulse_binding::context::subscribe::{Facility, InterestMaskSet};
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::{Proplist};
use std::thread;
use crate::{Electrode, make_icon};

// Based upon https://github.com/greshake/i3status-rust/blob/2e5e0474b9df1e9d3e547bc10550a341e1ea53f4/src/blocks/sound/pulseaudio.rs

struct Connection {
    mainloop: Mainloop,
    context: Context
}

impl Connection {
    fn new() -> Self {
        let mut proplist = Proplist::new().expect("could not create PulseAudio proplist");
        proplist.set_str("electrode", "electrode")
            .expect("could not set pulseaudio APPLICATION_NAME property");

        let mainloop = Mainloop::new().expect("could not create PulseAudio mainloop");

        let mut context = Context::new_with_proplist(&mainloop, "electrode_context", &proplist)
            .expect("could not create PulseAudio context");

        context.connect(None, FlagSet::NOFLAGS, None)
            .expect("could not connect to PulseAudio context");

        let mut connection = Connection { mainloop, context };

        // Wait for context to be ready
        loop {
            connection.iterate(false);

            match connection.context.get_state() {
                State::Ready => { break; },
                State::Failed => { panic!("PulseAudio context failed"); },
                State::Terminated => { panic!("PulseAudio context terminated"); },
                _ => {}
            }
        }

        connection
    }

    fn iterate(&mut self, blocking: bool) {
        match self.mainloop.iterate(blocking) {
            IterateResult::Quit(_) => {
                panic!("could not iterate PulseAudio connection")
            },
            IterateResult::Err(error) => {
                panic!("could not iterate PulseAudio connection: {}", error)
            },
            IterateResult::Success(_) => ()
        }
    }
}

pub struct Volume;

impl Electrode for Volume {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let mut connection = Connection::new();

            let sink_info_callback = |result: ListResult<&SinkInfo>| {
                match result {
                    ListResult::Item(sink) => {
                        sender.send(sink).expect("sending PulseAudio sink information through channel");
                    },
                    ListResult::End => {},
                    ListResult::Error => {
                        panic!("error fetching sink info from PulseAudio");
                    }
                }
            };

            let server_info_callback = |info: &ServerInfo| {
                if let Some(default_sink) = info.default_sink_name.as_ref() {
                    connection.context.introspect().get_sink_info_by_name(default_sink, sink_info_callback);
                }
            };
            connection.context.introspect().get_server_info(server_info_callback);

            let subscribe_callback = |facility, _, index| {
                match facility {
                    Some(Facility::Server) => {
                        connection.context.introspect().get_server_info(server_info_callback);
                    },
                    Some(Facility::Sink) => {
                        connection.context.introspect().get_sink_info_by_index(index, sink_info_callback);
                    },
                    _ => {}
                }
            };

            connection.context.set_subscribe_callback(Some(Box::new(subscribe_callback)));
            connection.context.subscribe(InterestMaskSet::SERVER | InterestMaskSet::SINK, |_| {});

            connection.mainloop.run().unwrap();
        });

        receiver.attach(None, clone!(
                @weak label =>
                @default-return Continue(false),
                move |sink| {
                    if sink.mute {
                        label.set_text("Mute");
                    } else {
                        let text = format!("{}%", sink.volume.avg());
                        label.set_text(&text);
                    }

                    Continue(true)
                }
        ));

        Volume
    }

    fn refresh(&mut self) {
    }
}
