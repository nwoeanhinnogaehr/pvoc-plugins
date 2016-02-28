use pvoc::{PhaseVocoder, Bin};
use ladspa::{self, PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};

struct PitchShifter {
    pvoc: PhaseVocoder,
}

impl PitchShifter {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(PitchShifter { pvoc: PhaseVocoder::new(2, sample_rate as f64, 10, 32) })
    }

    fn process(rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        for i in 0..channels {
            for j in 0..bins {
                let index = ((j as f64) * rate) as usize;
                if index < output[i].len() {
                    output[i][index].freq = input[i][j].freq * rate;
                    output[i][index].amp = input[i][j].amp;
                }
            }
        }
    }
}

impl Plugin for PitchShifter {
    fn run<'a>(&mut self, _: usize, ports: &[&'a PortConnection<'a>]) {
        let input = vec![ports[0].unwrap_audio(), ports[1].unwrap_audio()];
        let mut outputl = ports[2].unwrap_audio_mut();
        let mut outputr = ports[3].unwrap_audio_mut();
        let mut output = vec![&mut outputl[..], &mut outputr[..]];
        let rate = *ports[4].unwrap_control();
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            PitchShifter::process(rate as f64, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        unique_id: 9401,
        label: "pvoc_shift",
        properties: ladspa::PROP_NONE,
        name: "pvoc pitch shift",
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
                        name: "Shift",
                        desc: PortDescriptor::ControlInput,
                        hint: None,
                        default: Some(DefaultValue::Value1),
                        lower_bound: Some(0.5),
                        upper_bound: Some(2.0),
                    }],
        new: PitchShifter::new,
    }
}
