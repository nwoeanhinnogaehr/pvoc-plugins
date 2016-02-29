#![macro_use]

extern crate ladspa;
extern crate pvoc;

use ladspa::{PluginDescriptor, Port};
use pvoc::Bin;

macro_rules! plugin {
    ($name:ident) => {
        use std::mem;
        use ladspa;

        #[allow(non_camel_case_types)]
        struct __plug {
            sample_rate: f64,
            bins: usize,
            time_div: usize,
            channels: usize,
            pvoc: PhaseVocoder,
            plugin: $name,
        }
        fn __new_plug(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
            let desc = $name::descriptor();
            let bins = 8;
            let time_div = 4;
            Box::new(__plug {
                sample_rate: sample_rate as f64,
                bins: bins,
                time_div: time_div,
                channels: desc.channels,
                pvoc: PhaseVocoder::new(desc.channels, sample_rate as f64, bins, time_div),
                plugin: $name::new(desc.channels, sample_rate as f64, bins, time_div),
            })
        }
        impl Plugin for __plug {
            fn run<'a>(&mut self, _: usize, ports: &[&'a PortConnection<'a>]) {
                let mut input = Vec::new();
                let mut output = Vec::new();
                for i in 0..self.channels {
                    input.push(ports[i].unwrap_audio());

                    // sorry. not spending the time to fix this when I'm almost certain it's OK.
                    output.push(unsafe {
                        mem::transmute(&ports[i + self.channels].unwrap_audio_mut()[..] as *const [f32])
                    });
                }
                let ports = &ports[self.channels*2..];
                let bins = *ports[0].unwrap_control() as usize;
                if bins != self.bins {
                    self.bins = bins;
                    self.pvoc = PhaseVocoder::new(self.channels, self.sample_rate, self.bins, self.time_div);
                    self.plugin = $name::new(self.channels, self.sample_rate, self.bins, self.time_div);
                }
                let sample_rate = self.sample_rate;
                let plugin = &mut self.plugin;
                let user_ports = ports[1..].iter().map(|&x| *x.unwrap_control() as f64).collect::<Vec<f64>>();
                self.pvoc.process(&input, &mut output, |channels: usize, bins: usize,
                                  input: &[Vec<Bin>], output: &mut [Vec<Bin>]| {
                                      plugin.process(&user_ports, sample_rate, channels, bins, input, output)
                                  });
            }
        }
        pub fn get_descriptor() -> PluginDescriptor {
            const INPUT_NAMES: &'static [&'static str] =
                &["Audio In 1", "Audio In 2", "Audio In 3", "Audio In 4"];
            const OUTPUT_NAMES: &'static [&'static str] =
                &["Audio Out 1", "Audio Out 2", "Audio Out 3", "Audio Out 4"];
            let pdesc = $name::descriptor();
            let mut desc =
                PluginDescriptor {
                    unique_id: 9400,
                    label: pdesc.name,
                    properties: ladspa::PROP_NONE,
                    name: pdesc.name,
                    maker: "Noah Weninger",
                    copyright: "None",
                    ports: vec![],
                    new: __new_plug,
                };
            for i in 0..pdesc.channels {
                desc.ports.push(Port {
                    name: INPUT_NAMES[i],
                    desc: PortDescriptor::AudioInput,
                    ..Default::default()
                });
            }
            for i in 0..pdesc.channels {
                desc.ports.push(Port {
                    name: OUTPUT_NAMES[i],
                    desc: PortDescriptor::AudioOutput,
                    ..Default::default()
                });
            }
            desc.ports.extend(&[Port {
                name: "Bins",
                desc: PortDescriptor::ControlInput,
                hint: Some(ladspa::HINT_INTEGER | ladspa::HINT_LOGARITHMIC),
                default: None,
                lower_bound: Some(4.0),
                upper_bound: Some(65536.0),
            }]);
            desc.ports.extend(&pdesc.ports);
            desc
        }
    };
}

mod binflipper;
mod pitchshifter;
mod centroid;
mod expavg;
mod modularamp;
mod formantshifter;
mod freqshifter;
mod domainxover;
mod timeblur;

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
        8 => Some(timeblur::get_descriptor()),
        _ => None,
    }
    .map(|mut x| {
        x.unique_id += index;
        x
    })
}

trait PVocPlugin {
    fn descriptor() -> PVocDescriptor;
    fn new(channels: usize, sample_rate: f64, bins: usize, time_div: usize) -> Self;
    fn process(&mut self,
               ports: &[f64],
               sample_rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]);
}

struct PVocDescriptor {
    name: &'static str,
    channels: usize,
    ports: Vec<Port>,
}
