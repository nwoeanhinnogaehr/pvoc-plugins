use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

plugin!(BinFlipper);

struct BinFlipper;

impl PVocPlugin for BinFlipper {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc bin flipper",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Nyquist multipliter",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value1),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> BinFlipper {
        BinFlipper
    }
    fn process(&mut self,
               params: &[f64],
               sample_rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let mult = params[0];
        let freq_per_bin = sample_rate / (bins as f64) * mult;
        for i in 0..channels {
            for j in 0..bins {
                let expect = freq_per_bin * (j as f64) + freq_per_bin / 2.0;
                let new = -(input[i][j].freq - expect) + expect;
                output[i][j].amp = input[i][j].amp;
                output[i][j].freq = new;
            }
        }
    }
}
