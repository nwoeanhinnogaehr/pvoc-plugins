use super::{lerp, PVocDescriptor, PVocPlugin};
use ladspa::{DefaultValue, Plugin, PluginDescriptor, Port, PortConnection, PortDescriptor};
use pvoc::{Bin, PhaseVocoder};

const MAX_LENGTH: usize = 2000;

plugin!(Repeater);

struct Repeater {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
}

impl PVocPlugin for Repeater {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc repeater",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![
                Port {
                    name: "Length",
                    desc: PortDescriptor::ControlInput,
                    hint: Some(ladspa::HINT_INTEGER),
                    default: Some(DefaultValue::Value0),
                    lower_bound: Some(1.0),
                    upper_bound: Some(MAX_LENGTH as f32),
                },
                Port {
                    name: "Freq hold",
                    desc: PortDescriptor::ControlInput,
                    hint: None,
                    default: Some(DefaultValue::Value0),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                },
                Port {
                    name: "Amp hold",
                    desc: PortDescriptor::ControlInput,
                    hint: None,
                    default: Some(DefaultValue::Value0),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                },
                Port {
                    name: "Decay",
                    desc: PortDescriptor::ControlInput,
                    hint: None,
                    default: Some(DefaultValue::Value1),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                },
                Port {
                    name: "Mix",
                    desc: PortDescriptor::ControlInput,
                    hint: None,
                    default: Some(DefaultValue::Value1),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                },
            ],
        }
    }

    fn new(channels: usize, _: f64, bins: usize, _: usize) -> Repeater {
        Repeater {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; MAX_LENGTH],
            time: 0,
        }
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
        let length = ports[0] as usize;
        let freq_hold = ports[1];
        let amp_hold = ports[2];
        let decay = ports[3];
        let mix = ports[4];

        self.time %= length;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j].amp =
                    lerp(self.buffer[self.time][i][j].amp, input[i][j].amp, amp_hold);
                self.buffer[self.time][i][j].freq = lerp(
                    self.buffer[self.time][i][j].freq,
                    input[i][j].freq,
                    freq_hold,
                );
                output[i][j].amp = lerp(self.buffer[self.time][i][j].amp, input[i][j].amp, mix);
                output[i][j].freq = self.buffer[self.time][i][j].freq;
                self.buffer[self.time][i][j].amp *= decay;
            }
        }
        self.time += 1;
    }
}
