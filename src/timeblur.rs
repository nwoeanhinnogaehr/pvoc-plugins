use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};
use super::{PVocPlugin, PVocDescriptor};

plugin!(TimeBlur);

struct TimeBlur {
    buffer: Vec<Vec<Bin>>,
}

impl PVocPlugin for TimeBlur {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc time blur",
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
                        },
                        Port {
                            name: "Amplitude high replace mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Amplitude low replace mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: None,
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }

    fn new(channels: usize, _: f64, bins: usize, _: usize) -> TimeBlur {
        TimeBlur { buffer: vec![vec![Bin::new(0.0, 0.0); bins]; channels] }
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
        let replace_high = ports[4];
        let replace_low = ports[5];
        let mut buffer = &mut self.buffer;
        for i in 0..channels {
            for j in 0..bins {
                buffer[i][j].freq = buffer[i][j].freq * freq_alpha +
                                    input[i][j].freq * (1.0 - freq_alpha);
                buffer[i][j].amp = buffer[i][j].amp * amp_alpha +
                                   input[i][j].amp * (1.0 - amp_alpha);
                if input[i][j].amp > buffer[i][j].amp {
                    buffer[i][j].amp = input[i][j].amp * replace_high +
                                       buffer[i][j].amp * (1.0 - replace_high);
                }
                if input[i][j].amp < buffer[i][j].amp {
                    buffer[i][j].amp = input[i][j].amp * replace_low +
                                       buffer[i][j].amp * (1.0 - replace_low);
                }
                output[i][j].freq = buffer[i][j].freq * freq_mix +
                                    input[i][j].freq * (1.0 - freq_mix);
                output[i][j].amp = buffer[i][j].amp * amp_mix + input[i][j].amp * (1.0 - amp_mix);
            }
        }
    }
}
