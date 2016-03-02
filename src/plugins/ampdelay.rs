use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor, lerp};

plugin!(AmpDelay);

struct AmpDelay {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    max_delay: usize,
}

impl PVocPlugin for AmpDelay {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc amplitude scaled delay",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Delay",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(2000.0),
                        },
                        Port {
                            name: "Max delay",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value100),
                            lower_bound: Some(1.0),
                            upper_bound: Some(2000.0),
                        },
                        Port {
                            name: "Frequency mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value1),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Amplitude mix",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value1),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Frequency feedback",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        },
                        Port {
                            name: "Amplitude feedback",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }

    fn new(channels: usize, _: f64, bins: usize, _: usize) -> AmpDelay {
        AmpDelay {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; 0],
            time: 0,
            max_delay: 0,
        }
    }

    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let delay = ports[0];
        let max_delay = ports[1] as usize;
        let freq_mix = ports[2];
        let amp_mix = ports[3];
        let freq_feed = ports[4];
        let amp_feed = ports[5];
        let mut buffer = &mut self.buffer;

        if max_delay != self.max_delay {
            self.max_delay = max_delay;
            buffer.resize(max_delay, vec![vec![Bin::new(0.0, 0.0); bins]; channels]);
        }

        self.time %= max_delay;
        for i in 0..channels {
            for j in 0..bins {
                let bin_delay = ((input[i][j].amp + 1.0).log2() * delay) as usize;
                buffer[(self.time + bin_delay) % max_delay][i][j] = input[i][j];
                output[i][j].amp = lerp(buffer[self.time][i][j].amp, input[i][j].amp, amp_mix);
                output[i][j].freq = lerp(buffer[self.time][i][j].freq, input[i][j].freq, freq_mix);
                buffer[self.time][i][j].freq *= freq_feed;
                buffer[self.time][i][j].amp *= amp_feed;
            }
        }
        self.time += 1;
    }
}
