#![feature(type_ascription)]
extern crate ladspa;
extern crate pvoc;

use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin};

mod binflipper;
mod pitchshifter;
mod centroid;
mod expavg;
mod modularamp;
mod formantshifter;
mod freqshifter;
mod domainxover;

#[no_mangle]
pub fn get_ladspa_descriptor(index: u64) -> Option<PluginDescriptor> {
    match index {
        0 => Some(binflipper::get_descriptor()),
        1 => Some(pitchshifter::get_descriptor()),
        2 => Some(centroid::get_descriptor()),
        3 => Some(expavg::get_descriptor()),
        4 => Some(modularamp::get_descriptor()),
        5 => Some(formantshifter::get_descriptor()),
        6 => Some(freqshifter::get_descriptor()),
        7 => Some(domainxover::get_descriptor()),
        _ => None,
    }
    .map(|mut x| {
        x.unique_id += index;
        x
    })
}

pub fn base_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9400,
        label: "NONE_FIXME",
        properties: ladspa::PROP_NONE,
        name: "NONE FIXME",
        maker: "Noah Weninger",
        copyright: "None",
        ports: vec![Port {
                        name: "Left Audio In",
                        desc: PortDescriptor::AudioInput,
                        ..Default::default()
                    },
                    Port {
                        name: "Right Audio In",
                        desc: PortDescriptor::AudioInput,
                        ..Default::default()
                    },
                    Port {
                        name: "Left Audio Out",
                        desc: PortDescriptor::AudioOutput,
                        ..Default::default()
                    },
                    Port {
                        name: "Right Audio Out",
                        desc: PortDescriptor::AudioOutput,
                        ..Default::default()
                    },
                    Port {
                        name: "Bins",
                        desc: PortDescriptor::ControlInput,
                        hint: Some(ladspa::HINT_INTEGER),
                        default: None,
                        lower_bound: Some(2.0),
                        upper_bound: Some(16.0),
                    }],
        new: new_unimplemented,
    }
}

fn new_unimplemented(_: &PluginDescriptor, _: u64) -> Box<Plugin + Send> {
    unimplemented!();
}
