use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

plugin!(Gate);

struct Gate;

impl PVocPlugin for Gate {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc gate",
            channels: 1,
            ports: vec![Port {
                            name: "Gate",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Minimum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(8.0),
                        },
                        Port {
                            name: "Duck",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Maximum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(8.0),
                        }],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> Gate {
        Gate
    }
    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let gate = ports[0];
        let duck = ports[1];
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;
                let amp = (input[i][j].amp + 1.0).log2();
                // TODO smooth it out a bit at the boundary
                output[i][j].amp = if amp < gate || amp > duck {
                    0.0
                } else {
                    input[i][j].amp
                }
            }
        }
    }
}
