use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

plugin!(SlopeFilter);

struct SlopeFilter {
    buffer: Vec<Vec<Bin>>,
}

impl PVocPlugin for SlopeFilter {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc slope filter",
            channels: 1,
            ports: vec![Port {
                            name: "Freq min",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Minimum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(0.1),
                        },
                        Port {
                            name: "Freq max",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Maximum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(0.1),
                        },
                        Port {
                            name: "Amp min",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Minimum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(8.0),
                        },
                        Port {
                            name: "Amp max",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Maximum),
                            lower_bound: Some(0.0),
                            upper_bound: Some(8.0),
                        }],
        }
    }
    fn new(channels: usize, _: f64, bins: usize, _: usize) -> SlopeFilter {
        SlopeFilter { buffer: vec![vec![Bin::new(0.0, 0.0); bins]; channels] }
    }
    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_min = ports[0];
        let freq_max = ports[1];
        let amp_min = ports[2];
        let amp_max = ports[3];
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;

                let amp_slope = ((input[i][j].amp + 1.0).log2() -
                                 (self.buffer[i][j].amp + 1.0).log2())
                                    .abs();
                let freq_slope = ((input[i][j].freq + 1.0).log2() -
                                  (self.buffer[i][j].freq + 1.0).log2())
                                     .abs();
                output[i][j].amp = if amp_slope < amp_min || amp_slope > amp_max ||
                                      freq_slope < freq_min ||
                                      freq_slope > freq_max {
                    0.0
                } else {
                    input[i][j].amp
                };
                self.buffer[i][j] = input[i][j];
            }
        }
    }
}
