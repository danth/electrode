use gtk::prelude::*;
use gtk::glib::clone;
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::context::introspect::{ServerInfo, SinkInfo};
use libpulse_binding::context::subscribe::{Facility, InterestMaskSet};
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::{Proplist};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Electrode, make_icon};

fn connect_to_pulseaudio() -> (Mainloop, Context) {
    let mut proplist = Proplist::new().expect("could not create PulseAudio proplist");
    proplist.set_str("electrode", "electrode")
        .expect("could not set pulseaudio APPLICATION_NAME property");

    let mut mainloop = Mainloop::new().expect("could not create PulseAudio mainloop");

    let mut context = Context::new_with_proplist(&mainloop, "electrode_context", &proplist)
        .expect("could not create PulseAudio context");
    context.connect(None, FlagSet::NOFLAGS, None)
        .expect("could not connect to PulseAudio context");

    // Wait for context to be ready
    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(_) => panic!("could not iterate PulseAudio connection"),
            IterateResult::Err(error) => panic!("could not iterate PulseAudio connection: {}", error),
            IterateResult::Success(_) => ()
        }

        match context.get_state() {
            State::Ready => { break; },
            State::Failed => { panic!("PulseAudio context failed"); },
            State::Terminated => { panic!("PulseAudio context terminated"); },
            _ => {}
        }
    }

    (mainloop, context)
}

#[derive(Clone)]
enum VolumeSetting {
    Muted,
    Volume(libpulse_binding::volume::Volume)
}

impl From<&SinkInfo<'_>> for VolumeSetting {
    fn from(sink: &SinkInfo) -> Self {
        if sink.mute {
            VolumeSetting::Muted
        } else {
            VolumeSetting::Volume(sink.volume.avg())
        }
    }
}

// HACK: Lots of Arcs, Mutexes and cloning in the PulseAudio client

#[derive(Clone)]
struct Client {
    default_sink: Arc<Mutex<Option<String>>>,
    sinks: Arc<Mutex<HashMap<String, VolumeSetting>>>
}

impl Client {
    fn new() -> Self {
        let client = Client {
            default_sink: Arc::new(Mutex::new(None)),
            sinks: Arc::new(Mutex::new(HashMap::new()))
        };

        thread::spawn(clone!(@strong client => move || {
            let (mut mainloop, context) = connect_to_pulseaudio();
            let context = Rc::new(RefCell::new(context));

            context.borrow_mut().introspect().get_server_info(clone!(
                @strong client => move |info| client.ingest_server_info(info)
            ));
            context.borrow_mut().introspect().get_sink_info_list(clone!(
                @strong client => move |info| client.ingest_sink_info(info)
            ));

            let subscribe_callback = clone!(@strong context => move |facility, _, index| {
                match facility {
                    Some(Facility::Server) => {
                        context.borrow_mut().introspect().get_server_info(clone!(
                            @strong client => move |info| client.ingest_server_info(info)
                        ));
                    },
                    Some(Facility::Sink) => {
                        context.borrow_mut().introspect().get_sink_info_by_index(index, clone!(
                            @strong client => move |result| client.ingest_sink_info(result)
                        ));
                    },
                    _ => {}
                }
            });

            context.borrow_mut().set_subscribe_callback(Some(Box::new(subscribe_callback)));
            context.borrow_mut().subscribe(InterestMaskSet::SERVER | InterestMaskSet::SINK, |_| {});

            mainloop.run().unwrap();
        }));

        client
    }

    fn ingest_server_info(&self, info: &ServerInfo) {
        if let Some(sink_name) = info.default_sink_name.as_ref() {
            let mut default_sink = self.default_sink.lock().unwrap();
            *default_sink = Some(sink_name.to_string());
        }
    }

    fn ingest_sink_info(&self, result: ListResult<&SinkInfo>) {
        match result {
            ListResult::Item(sink) => {
                if let Some(name) = &sink.name {
                    let mut sinks = self.sinks.lock().unwrap();
                    sinks.insert(name.to_string(), sink.into());
                }
            },
            ListResult::End => {},
            ListResult::Error => {
                panic!("error fetching sink info from PulseAudio");
            }
        }
    }

    fn get_default_sink_volume(&self) -> Option<VolumeSetting> {
        self.default_sink.lock().unwrap().as_ref().and_then(|default_sink| {
            let sinks = self.sinks.lock().unwrap();
            sinks.get(default_sink).map(|sink| sink.clone())
        })
    }
}

pub struct Volume {
    box_: gtk::Box,
    label: gtk::Label,
    client: Client
}

impl Electrode for Volume {
    fn initialize(parent: &gtk::Box) -> Self {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");

        Volume {
            box_, label,
            client: Client::new()
        }
    }

    fn refresh(&mut self) {
        if let Some(volume) = self.client.get_default_sink_volume() {
            match volume {
                VolumeSetting::Muted => {
                    self.label.set_label("Mute");
                },
                VolumeSetting::Volume(volume) => {
                    self.label.set_label(&volume.to_string());
                }
            }

            self.box_.set_visible(true);
        } else {
            self.box_.set_visible(false);
        }
    }
}
