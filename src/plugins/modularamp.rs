use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

plugin!(ModularAmp);

struct ModularAmp;

impl PVocPlugin for ModularAmp {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc amplitude modulus",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Mod",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value1),
                            lower_bound: Some(0.0),
                            upper_bound: Some(25.0),
                        }],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> ModularAmp {
        ModularAmp
    }
    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let factor = ports[0].recip();
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;
                output[i][j].amp = fmod(input[i][j].amp, factor);
            }
        }
    }
}

fn fmod(a: f64, b: f64) -> f64 {
    if b.is_infinite() {
        a
    } else {
        a - b * (a / b).floor()
    }
}
