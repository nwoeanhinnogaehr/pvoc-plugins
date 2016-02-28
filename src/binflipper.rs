use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection, DefaultValue};

struct BinFlipper {
    pvoc: PhaseVocoder,
    sample_rate: f64,
    bins: usize,
}

impl BinFlipper {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(BinFlipper {
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
        let bins = *ports[4].unwrap_control() as usize;
        if bins != self.bins {
            self.bins = bins;
            self.pvoc = PhaseVocoder::new(2, self.sample_rate, bins, 4);
        }
        let mult = *ports[5].unwrap_control() as f64;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            BinFlipper::process(sample_rate * mult, channels, bins, input, output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    let mut desc = super::base_descriptor();
    desc.label = "pvoc_bin_flipper";
    desc.name = "pvoc bin flipper";
    desc.ports.push(Port {
        name: "Nyquist multipliter",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Value1),
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc.new = BinFlipper::new;
    desc
}
