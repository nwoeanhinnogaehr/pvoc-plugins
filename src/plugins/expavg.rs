use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};
use super::{PVocPlugin, PVocDescriptor};

plugin!(ExpAvg);

struct ExpAvg;

impl PVocPlugin for ExpAvg {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc upwards sweeping exponential averager",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Frequency alpha",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Amplitude alpha",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Freqency mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Amplitude mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }

    fn new(_: usize, _: f64, _: usize, _: usize) -> ExpAvg {
        ExpAvg
    }

    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_alpha = ports[0];
        let amp_alpha = ports[1];
        let freq_mix = ports[2];
        let amp_mix = ports[3];
        for i in 0..channels {
            let mut avg_freq = input[i][0].freq;
            let mut avg_amp = input[i][0].amp;
            for j in 0..bins {
                output[i][j].freq = avg_freq * freq_mix + input[i][j].freq * (1.0 - freq_mix);
                output[i][j].amp = avg_amp * amp_mix + input[i][j].amp * (1.0 - amp_mix);
                avg_freq = avg_freq * freq_alpha + input[i][j].freq * (1.0 - freq_alpha);
                avg_amp = avg_amp * amp_alpha + input[i][j].amp * (1.0 - amp_alpha);
            }
        }
    }
}
