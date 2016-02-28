use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};

struct ExpAvg {
    pvoc: PhaseVocoder,
    sample_rate: f64,
    bins: usize,
}

impl ExpAvg {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(ExpAvg {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 8, 4),
            sample_rate: sample_rate as f64,
            bins: 8,
        })
    }

    fn process(freq_alpha: f64,
               amp_alpha: f64,
               freq_mix: f64,
               amp_mix: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
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

impl Plugin for ExpAvg {
    fn run<'a>(&mut self, _: usize, ports: &[&'a PortConnection<'a>]) {
        let input = vec![ports[0].unwrap_audio(), ports[1].unwrap_audio()];
        let mut outputl = ports[2].unwrap_audio_mut();
        let mut outputr = ports[3].unwrap_audio_mut();
        let mut output = vec![&mut outputl[..], &mut outputr[..]];
        let bins = *ports[4].unwrap_control() as usize;
        let freq_alpha = *ports[5].unwrap_control() as f64;
        let amp_alpha = *ports[6].unwrap_control() as f64;
        let freq_mix = *ports[7].unwrap_control() as f64;
        let amp_mix = *ports[8].unwrap_control() as f64;
        if bins != self.bins {
            self.bins = bins;
            self.pvoc = PhaseVocoder::new(2, self.sample_rate, bins, 4);
        }
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            ExpAvg::process(freq_alpha,
                            amp_alpha,
                            freq_mix,
                            amp_mix,
                            channels,
                            bins,
                            input,
                            output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9403,
        label: "pvoc_exponential_averager",
        properties: ladspa::PROP_NONE,
        name: "pvoc upwards sweeping exponential averager",
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
                        name: "Freqency Alpha",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: None,
                        lower_bound: Some(0.0),
                        upper_bound: Some(1.0),
                    },
                    Port {
                        name: "Amplitude Alpha",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: None,
                        lower_bound: Some(0.0),
                        upper_bound: Some(1.0),
                    },
                    Port {
                        name: "Freqency Mix",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: None,
                        lower_bound: Some(0.0),
                        upper_bound: Some(1.0),
                    },
                    Port {
                        name: "Amplitude Mix",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: None,
                        lower_bound: Some(0.0),
                        upper_bound: Some(1.0),
                    }],
        new: ExpAvg::new,
    }
}
