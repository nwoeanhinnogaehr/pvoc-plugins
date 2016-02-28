use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};

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
    let mut desc = super::base_descriptor();
    desc.label = "pvoc_formant_shifter";
    desc.name = "pvoc formant shifter";
    desc.ports.push(Port {
        name: "Shift",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Value1),
        lower_bound: Some(0.0),
        upper_bound: Some(8.0),
    });
    desc.new = FormantShifter::new;
    desc
}
