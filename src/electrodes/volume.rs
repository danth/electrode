use gtk::prelude::*;
use gtk::glib::{self, clone};
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::context::introspect::{ServerInfo, SinkInfo};
use libpulse_binding::context::subscribe::InterestMaskSet;
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::{Proplist};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::electrodes::{Electrode, make_icon};

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

#[derive(Clone, Debug)]
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
    sender: Arc<glib::Sender<Option<VolumeSetting>>>,
    context: Arc<Mutex<Option<Context>>>
}

macro_rules! borrow_context {
    ($self:ident) => {
        $self.context.lock().unwrap().as_mut().expect("PulseAudio context has not been initialised")
    }
}

impl Client {
    fn new(sender: glib::Sender<Option<VolumeSetting>>) -> Self {
        let client = Client {
            sender: Arc::new(sender),
            context: Arc::new(Mutex::new(None))
        };

        thread::spawn(clone!(@strong client => move || {
            let (mut mainloop, context) = connect_to_pulseaudio();

            {
                let mut context_lock = client.context.lock().unwrap();
                *context_lock = Some(context);
            }

            borrow_context!(client).introspect().get_server_info(clone!(
                @strong client =>
                move |info| client.ingest_server_info(info)
            ));

            let subscribe_callback = clone!(
                @strong client =>
                move |_, _, _| {
                    borrow_context!(client).introspect().get_server_info(clone!(
                        @strong client =>
                        move |info| client.ingest_server_info(info)
                    ));
                }
            );

            borrow_context!(client)
                .set_subscribe_callback(Some(Box::new(subscribe_callback)));
            borrow_context!(client)
                .subscribe(InterestMaskSet::SERVER | InterestMaskSet::SINK, |_| {});

            mainloop.run().unwrap();
        }));

        client
    }

    fn ingest_server_info(&self, info: &ServerInfo) {
        match info.default_sink_name.as_ref() {
            Some(sink_name) => {
                borrow_context!(self).introspect().get_sink_info_by_name(&sink_name, clone!(
                    @strong self as client => move |info| client.ingest_sink_info(info)
                ));
            },
            None => {
                self.send_event(None);
            }
        }
    }

    fn ingest_sink_info(&self, result: ListResult<&SinkInfo>) {
        match result {
            ListResult::Item(sink) => self.send_event(Some(sink.into())),
            ListResult::End => (),
            ListResult::Error => panic!("error fetching sink info from PulseAudio")
        }
    }

    fn send_event(&self, volume: Option<VolumeSetting>) {
        self.sender.send(volume).expect("sending volume change through channel");
    }
}

pub struct Volume;

impl Electrode for Volume {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(&parent, "");
        box_.style_context().add_class("electrode");
        box_.set_visible(false);

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        Client::new(sender);

        receiver.attach(None, clone!(
            @weak label =>
            @default-return Continue(false),
            move |volume| {
                if let Some(volume) = volume {
                    match volume {
                        VolumeSetting::Muted => {
                            label.set_label("Mute");
                        },
                        VolumeSetting::Volume(volume) => {
                            label.set_label(&volume.to_string());
                        }
                    }

                    box_.set_visible(true);
                } else {
                    box_.set_visible(false);
                }

                Continue(true)
            }
        ));
    }
}
