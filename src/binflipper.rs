use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};

struct BinFlipper {
    pvoc: PhaseVocoder,
    sample_rate: f64,
}

impl BinFlipper {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(BinFlipper {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 8, 4),
            sample_rate: sample_rate as f64,
        })
    }

    fn process(sample_rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_per_bin = sample_rate / (bins as f64);
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

impl Plugin for BinFlipper {
    fn run<'a>(&mut self, _: usize, ports: &[&'a PortConnection<'a>]) {
        let input = vec![ports[0].unwrap_audio(), ports[1].unwrap_audio()];
        let mut outputl = ports[2].unwrap_audio_mut();
        let mut outputr = ports[3].unwrap_audio_mut();
        let mut output = vec![&mut outputl[..], &mut outputr[..]];
        let sample_rate = self.sample_rate;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            BinFlipper::process(sample_rate, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9400,
        label: "pvoc_bin_flipper",
        properties: ladspa::PROP_NONE,
        name: "pvoc bin flipper",
        maker: "Noah Weninger",
        copyright: "None",
        ports: vec![Port {
                        name: "Left Audio In",
                        desc: PortDescriptor::AudioInput,
                        ..Default::default()
                    },
                    Port {
                        name: "Right Audio In",
                        desc: PortDescriptor::AudioInput,
                        ..Default::default()
                    },
                    Port {
                        name: "Left Audio Out",
                        desc: PortDescriptor::AudioOutput,
                        ..Default::default()
                    },
                    Port {
                        name: "Right Audio Out",
                        desc: PortDescriptor::AudioOutput,
                        ..Default::default()
                    }],
        new: BinFlipper::new,
    }
}
