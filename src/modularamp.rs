use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};

fn fmod(a: f64, b: f64) -> f64 {
    if b.is_infinite() {
        a
    } else {
        a - b * (a / b).floor()
    }
}

struct ModularAmp {
    pvoc: PhaseVocoder,
    bins: usize,
    sample_rate: f64,
}

impl ModularAmp {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(ModularAmp {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 12, 4),
            bins: 12,
            sample_rate: sample_rate as f64,
        })
    }

    fn process(factor: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;
                output[i][j].amp = fmod(input[i][j].amp, factor);
            }
        }
    }
}

impl Plugin for ModularAmp {
    fn run<'a>(&mut self, _: usize, ports: &[&'a PortConnection<'a>]) {
        let input = vec![ports[0].unwrap_audio(), ports[1].unwrap_audio()];
        let mut outputl = ports[2].unwrap_audio_mut();
        let mut outputr = ports[3].unwrap_audio_mut();
        let mut output = vec![&mut outputl[..], &mut outputr[..]];
        let bins = *ports[4].unwrap_control() as usize;
        if bins != self.bins {
            self.bins = bins;
            self.pvoc = PhaseVocoder::new(2, self.sample_rate, bins, 4);
        }
        let factor = (*ports[5].unwrap_control() as f64).recip();
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            ModularAmp::process(factor, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9404,
        label: "pvoc_modamp",
        properties: ladspa::PROP_NONE,
        name: "pvoc amplitude modulus",
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
                    },
                    Port {
                        name: "Bins",
                        desc: PortDescriptor::ControlInput,
                        hint: Some(ladspa::HINT_INTEGER),
                        default: None,
                        lower_bound: Some(2.0),
                        upper_bound: Some(16.0),
                    },
                    Port {
                        name: "Mod",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: Some(DefaultValue::Value0),
                        lower_bound: Some(0.0),
                        upper_bound: Some(25.0),
                    }],
        new: ModularAmp::new,
    }
}
