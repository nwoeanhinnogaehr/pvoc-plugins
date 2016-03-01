use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

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
            ports: vec![Port {
                            name: "Length",
                            desc: PortDescriptor::ControlInput,
                            hint: Some(ladspa::HINT_INTEGER),
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(1.0),
                            upper_bound: Some(MAX_LENGTH as f32),
                        },
                        Port {
                            name: "Hold",
                            desc: PortDescriptor::ControlInput,
                            hint: None,
                            default: Some(DefaultValue::Value0),
                            lower_bound: Some(0.0),
                            upper_bound: Some(1.0),
                        }],
        }
    }

    fn new(channels: usize, _: f64, bins: usize, _: usize) -> Repeater {
        Repeater {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; MAX_LENGTH],
            time: 0,
        }
    }

    fn process(&mut self,
               ports: &[f64],
               _: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let length = ports[0] as usize;
        let hold = ports[1];

        self.time %= length;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j].amp = self.buffer[self.time][i][j].amp * hold +
                                                   input[i][j].amp * (1.0 - hold);
                self.buffer[self.time][i][j].freq = self.buffer[self.time][i][j].freq * hold +
                                                    input[i][j].freq * (1.0 - hold);
                output[i][j] = self.buffer[self.time][i][j];
            }
        }
        self.time += 1;
    }
}
