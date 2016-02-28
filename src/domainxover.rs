use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};

struct DomainXOver {
    pvoc: PhaseVocoder,
    bins: usize,
    sample_rate: f64,
}

impl DomainXOver {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(DomainXOver {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 12, 4),
            bins: 12,
            sample_rate: sample_rate as f64,
        })
    }

    fn process(sample_rate: f64,
               add: f64,
               shift: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_per_bin = sample_rate / (bins as f64);
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq + shift * freq_per_bin + input[i][j].amp * add;
                output[i][j].amp = input[i][j].amp;
            }
        }
    }
}

impl Plugin for DomainXOver {
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
        let add = *ports[5].unwrap_control() as f64;
        let shift = *ports[6].unwrap_control() as f64;
        let sample_rate = self.sample_rate;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            DomainXOver::process(sample_rate, add, shift, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9407,
        label: "pvoc_domain_xover",
        properties: ladspa::PROP_NONE,
        name: "pvoc domain crossover",
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
                        name: "Add",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: Some(DefaultValue::Value0),
                        lower_bound: Some(0.0),
                        upper_bound: Some(25.0),
                    },
                    Port {
                        name: "Shift",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: Some(DefaultValue::Value0),
                        lower_bound: Some(0.0),
                        upper_bound: Some(1.0),
                    }],
        new: DomainXOver::new,
    }
}
