use super::{PVocDescriptor, PVocPlugin};
use ladspa::{DefaultValue, Plugin, PluginDescriptor, Port, PortConnection, PortDescriptor};
use pvoc::{Bin, PhaseVocoder};

const MAX_LENGTH: usize = 4096;

plugin!(Scrambler);

struct Scrambler {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    k: usize,
}

impl PVocPlugin for Scrambler {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc scrambler",
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
                    name: "Increment",
                    desc: PortDescriptor::ControlInput,
                    hint: Some(ladspa::HINT_INTEGER),
                    default: Some(DefaultValue::Value0),
                    lower_bound: Some(1.0),
                    upper_bound: Some(MAX_LENGTH as f32),
                },
            ],
        }
    }

    fn new(channels: usize, _: f64, bins: usize, _: usize) -> Scrambler {
        Scrambler {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; MAX_LENGTH],
            time: 0,
            k: 0,
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
        let increment = ports[1] as usize;

        self.time %= length;
        self.k %= length;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j].amp = input[i][j].amp;
                self.buffer[self.time][i][j].freq = input[i][j].freq;
                output[i][j].amp = self.buffer[self.k][i][j].amp;
                output[i][j].freq = self.buffer[self.k][i][j].freq;
            }
        }
        self.time += 1;
        self.k += increment;
    }
}
