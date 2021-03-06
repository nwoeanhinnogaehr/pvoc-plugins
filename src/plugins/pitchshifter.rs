use super::{PVocDescriptor, PVocPlugin};
use ladspa::{DefaultValue, Plugin, PluginDescriptor, Port, PortConnection, PortDescriptor};
use pvoc::{Bin, PhaseVocoder};

plugin!(PitchShifter);

struct PitchShifter;

impl PVocPlugin for PitchShifter {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc pitch shifter",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                name: "Shift",
                desc: PortDescriptor::ControlInput,
                hint: None,
                default: Some(DefaultValue::Value1),
                lower_bound: Some(0.0),
                upper_bound: Some(8.0),
            }],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> PitchShifter {
        PitchShifter
    }
    fn process(
        &mut self,
        ports: &[f64],
        _: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let shift = ports[0];
        for i in 0..channels {
            for j in 0..bins / 2 {
                let index = ((j as f64) * shift) as usize;
                if index < bins / 2 {
                    output[i][index].freq = input[i][j].freq * shift;
                    output[i][index].amp += input[i][j].amp;
                }
            }
        }
    }
}
