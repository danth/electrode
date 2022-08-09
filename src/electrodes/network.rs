use gtk::prelude::*;
use systemstat::{Platform, platform::PlatformImpl, System};
use crate::{Electrode, make_icon};

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

    fn refresh(self) -> Self {
        let totals = Totals::current(&self.system);
        let rates = self.previous_totals.rate_of_change(&totals);

        self.upload_label.set_label(&rates.upload.to_string_as(true).replace(' ', "\n"));
        self.download_label.set_label(&rates.download.to_string_as(true).replace(' ', "\n"));

        Network {
            previous_totals: totals,
            ..self
        }
    }
}
