use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, DefaultValue, Plugin, PortConnection};

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
               alpha: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        let freq_per_bin = sample_rate / (bins as f64);
        for i in 0..channels {
            let mut avg = input[i][0].amp;
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq + shift * freq_per_bin +
                                    ((avg - input[i][j].amp) * add);
                output[i][j].amp = input[i][j].amp;
                avg = avg * alpha + input[i][j].amp * (1.0 - alpha);
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
        let alpha = *ports[7].unwrap_control() as f64;
        let sample_rate = self.sample_rate;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            DomainXOver::process(sample_rate,
                                 add,
                                 shift,
                                 alpha,
                                 channels,
                                 bins,
                                 input,
                                 output)
        });
    }
}

pub fn get_descriptor() -> PluginDescriptor {
    let mut desc = super::base_descriptor();
    desc.label = "pvoc_domain_xover";
    desc.name = "pvoc domain xover";
    desc.new = DomainXOver::new;
    desc.ports.push(Port {
        name: "Add",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Value0),
        lower_bound: Some(0.0),
        upper_bound: Some(25.0),
    });
    desc.ports.push(Port {
        name: "Shift",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Value0),
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc.ports.push(Port {
        name: "Alpha",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: Some(DefaultValue::Middle),
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc
}
