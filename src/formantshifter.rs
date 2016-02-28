use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};

struct FormantShifter {
    pvoc: PhaseVocoder,
    sample_rate: f64,
    bins: usize,
}

impl FormantShifter {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(FormantShifter {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 8, 4),
            sample_rate: sample_rate as f64,
            bins: 8,
        })
    }

    fn process(shift: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        for i in 0..channels {
            for j in 0..bins {
                let index = ((j as f64) * shift) as usize;
                if index < bins {
                    output[i][j].amp = input[i][index].amp;
                }
                output[i][j].freq = input[i][j].freq;
            }
        }
    }
}

impl Plugin for FormantShifter {
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
        let shift = *ports[5].unwrap_control() as f64;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            FormantShifter::process(shift, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9405,
        label: "pvoc_formant_shifter",
        properties: ladspa::PROP_NONE,
        name: "pvoc formant shifter",
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
                        name: "Shift",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: Some(DefaultValue::Value1),
                        lower_bound: Some(0.0),
                        upper_bound: Some(8.0),
                    }],
        new: FormantShifter::new,
    }
}
