use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};
use super::{PVocPlugin, PVocDescriptor};

plugin!(DomainXOver);

struct DomainXOver;

impl PVocPlugin for DomainXOver {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc domain crossover",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Add",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(25.0),
                        },
                        Port {
                            name: "Shift",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Alpha",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Middle),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> DomainXOver {
        DomainXOver
    }
    fn process(&mut self,
               ports: &[f64],
               sample_rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_per_bin = sample_rate / (bins as f64);
        let add = ports[0];
        let shift = ports[1];
        let alpha = ports[2];
        for i in 0..channels {
            let mut avg = input[i][0].amp;
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq + shift * freq_per_bin +
                                    ((avg - input[i][j].amp) * add);
                output[i][j].amp = input[i][j].amp;
                avg = avg * alpha + input[i][j].amp * (1.0 - alpha);
            }
        }
    }
}
