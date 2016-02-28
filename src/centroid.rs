use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};

struct Centroid {
    pvoc: PhaseVocoder,
    sample_rate: f64,
    bins: usize,
}

impl Centroid {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(Centroid {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 8, 4),
            sample_rate: sample_rate as f64,
            bins: 8,
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
                output[i][j].amp = input[i][j].amp;
                output[i][j].freq = expect;
            }
        }
    }
}

impl Plugin for Centroid {
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
        let sample_rate = self.sample_rate;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            Centroid::process(sample_rate, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9402,
        label: "pvoc_centroid",
        properties: ladspa::PROP_NONE,
        name: "pvoc centroid",
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
                    }],
        new: Centroid::new,
    }
}
