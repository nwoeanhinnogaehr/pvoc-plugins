use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};
use super::{PVocPlugin, PVocDescriptor};

const SIZE: usize = 4;

plugin!(Stencil);

struct Stencil {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
}

impl PVocPlugin for Stencil {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc stencil",
            author: "Noah Weninger",
            channels: 1,
            ports: vec![Port {
                            name: "Stencil",
                            desc: PortDescriptor::ControlInput,
                            hint: Some(ladspa::HINT_INTEGER),
                            default: Some(DefaultValue::Value1),
                            lower_bound: Some(0.0),
                            upper_bound: Some(((1 << SIZE * SIZE) - 1) as f32),
                        }],
        }
    }
    fn new(channels: usize, _: f64, bins: usize, _: usize) -> Stencil {
        Stencil {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; SIZE],
            time: 0,
        }
    }
    fn process(&mut self,
               ports: &[f64],
               sample_rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let stencil = ports[0] as usize;
        let freq_per_bin = sample_rate / (bins as f64);
        self.time %= SIZE;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j] = input[i][j];
            }
            for j in 0..bins {
                let mut tmp = stencil;
                let mut ncontrib = 0;
                for x in 0..SIZE {
                    for y in 0..SIZE {
                        if tmp & 1 == 1 {
                            let bin = j + x;
                            if bin < 2 || bin >= bins - 2 {
                                continue;
                            }
                            let bin = bin - 2;
                            let frame = (self.time + SIZE - y) % SIZE;
                            output[i][j].amp += self.buffer[frame][i][bin].amp;
                            // TODO adjust frequency by bin difference
                            output[i][j].freq += self.buffer[frame][i][bin].freq;
                            ncontrib += 1;
                        }
                        tmp >>= 1;
                    }
                }
                if ncontrib > 0 {
                    // TODO configurable gain compensation
                    output[i][j].amp /= ncontrib as f64;
                    output[i][j].freq /= ncontrib as f64;
                }
            }
        }
        self.time += 1;
    }
}
