use gtk::prelude::*;
use systemstat::{ByteSize, Platform, platform::PlatformImpl, System};
use std::time::Instant;
use crate::{Electrode, make_icon};

#[derive(Debug)]
struct Totals {
    timestamp: Instant,
    upload: ByteSize,
    download: ByteSize
}

#[derive(Debug, Default)]
struct Rates {
    upload: ByteSize,
    download: ByteSize
}

impl Totals {
    fn current(system: &PlatformImpl) -> Self {
        let mut totals = Totals {
            timestamp: Instant::now(),
            upload: ByteSize::b(0),
            download: ByteSize::b(0)
        };

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
        let duration = future.timestamp.duration_since(self.timestamp);

        let upload_change = future.upload.as_u64() - self.upload.as_u64();
        let upload_rate = (upload_change as f64) / duration.as_secs_f64();
        let upload_rate = ByteSize::b(upload_rate.ceil() as u64);

        let download_change = future.download.as_u64() - self.download.as_u64();
        let download_rate = (download_change as f64) / duration.as_secs_f64();
        let download_rate = ByteSize::b(download_rate.ceil() as u64);

        Rates { upload: upload_rate, download: download_rate }
    }
}

pub struct Network {
    upload_label: gtk::Label,
    download_label: gtk::Label,
    system: PlatformImpl,
    previous_totals: Totals
}

impl Electrode for Network {
    fn initialize(parent: &gtk::Box) -> Self {
        let network_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        network_box.style_context().add_class("electrode");
        parent.add(&network_box);

        let (_, upload_label) = make_icon(&network_box, "");
        let (_, download_label) = make_icon(&network_box, "");

        let system = System::new();
        let totals = Totals::current(&system);

        Network {
            upload_label,
            download_label,
            system,
            previous_totals: totals
        }
    }

    fn refresh(&mut self) {
        let totals = Totals::current(&self.system);
        let rates = self.previous_totals.rate_of_change(&totals);

        self.upload_label.set_label(&rates.upload.to_string_as(true).replace(' ', "\n"));
        self.download_label.set_label(&rates.download.to_string_as(true).replace(' ', "\n"));

        self.previous_totals = totals;
    }
}
