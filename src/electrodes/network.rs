use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, platform::PlatformImpl, System};
use std::time::Duration;
use crate::electrodes::{Electrode, make_icon};

#[derive(Debug, Default)]
struct Totals {
    upload: systemstat::ByteSize,
    download: systemstat::ByteSize
}

#[derive(Debug, Default)]
struct Rates {
    upload: systemstat::ByteSize,
    download: systemstat::ByteSize
}

impl Totals {
    fn current(system: &PlatformImpl) -> Self {
        let mut totals = Totals::default();

        let networks = system.networks().expect("could not get list of networks");
        for network in networks.values() {

            let stats = system.network_stats(&network.name)
                .expect("could not get network statistics");

            totals.upload += stats.tx_bytes;
            totals.download += stats.rx_bytes;
        }

        totals
    }

    fn rate_of_change(&self, future: &Totals) -> Rates {
        Rates {
            upload: systemstat::ByteSize::b(future.upload.as_u64() - self.upload.as_u64()),
            download: systemstat::ByteSize::b(future.download.as_u64() - self.download.as_u64())
        }
    }
}

pub struct Network;

impl Electrode for Network {
    fn setup(parent: &gtk::Box) {
        let network_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        network_box.style_context().add_class("electrode");
        parent.add(&network_box);

        let (_, upload_label) = make_icon(&network_box, "");
        let (_, download_label) = make_icon(&network_box, "");

        glib::MainContext::default().spawn_local(clone!(
            @weak upload_label, @weak download_label =>
            async move {
                let system = System::new();
                let mut previous_totals = Totals::current(&system);

                loop {
                    task::sleep(Duration::from_secs(1)).await;

                    let totals = Totals::current(&system);
                    let rates = previous_totals.rate_of_change(&totals);

                    upload_label.set_label(&rates.upload.to_string_as(true).replace(' ', "\n"));
                    download_label.set_label(&rates.download.to_string_as(true).replace(' ', "\n"));

                    previous_totals = totals;
                }
            }
        ));
    }
}
