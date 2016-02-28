use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};

struct FreqShifter {
    pvoc: PhaseVocoder,
    bins: usize,
    sample_rate: f64,
}

impl FreqShifter {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(FreqShifter {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 10, 32),
            bins: 10,
            sample_rate: sample_rate as f64,
        })
    }

    fn process(rate: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        for i in 0..channels {
            for j in 0..bins {
                let index = ((j as f64) * rate) as usize;
                if index < bins {
                    output[i][index].freq = input[i][j].freq * rate;
                }
                output[i][j].amp = input[i][j].amp;
            }
        }
    }
}

impl Plugin for FreqShifter {
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
        let rate = *ports[5].unwrap_control();
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            FreqShifter::process(rate as f64, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    let mut desc = super::base_descriptor();
    desc.label = "pvoc_freq_shifter";
    desc.name = "pvoc frequency shifter";
    desc.ports.push(Port {
        name: "Shift",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Value1),
        lower_bound: Some(0.0),
        upper_bound: Some(8.0),
    });
    desc.new = FreqShifter::new;
    desc
}
