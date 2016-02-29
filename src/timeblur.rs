use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};

struct TimeBlur {
    pvoc: PhaseVocoder,
    sample_rate: f64,
    bins: usize,
    buffer: Vec<Vec<Bin>>,
}

impl TimeBlur {
    fn new(_: &PluginDescriptor, sample_rate: u64) -> Box<Plugin + Send> {
        Box::new(TimeBlur {
            pvoc: PhaseVocoder::new(2, sample_rate as f64, 8, 4),
            sample_rate: sample_rate as f64,
            bins: 8,
            buffer: Vec::new(),
        })
    }

    fn process(buffer: &mut [Vec<Bin>],
               freq_alpha: f64,
               amp_alpha: f64,
               freq_mix: f64,
               amp_mix: f64,
               channels: usize,
               bins: usize,
               input: &[Vec<Bin>],
               output: &mut [Vec<Bin>]) {
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = buffer[i][j].freq * freq_mix +
                                    input[i][j].freq * (1.0 - freq_mix);
                output[i][j].amp = buffer[i][j].amp * amp_mix + input[i][j].amp * (1.0 - amp_mix);
                buffer[i][j].freq = buffer[i][j].freq * freq_alpha +
                                    input[i][j].freq * (1.0 - freq_alpha);
                buffer[i][j].amp = buffer[i][j].amp * amp_alpha +
                                   input[i][j].amp * (1.0 - amp_alpha);
            }
        }
    }
}

impl Plugin for TimeBlur {
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
            self.buffer =
                vec![vec![Bin::new(0.0, 0.0); self.pvoc.num_bins()]; self.pvoc.num_channels()];
        }
        let buffer = &mut self.buffer;
        self.pvoc.process(&input, &mut output, |channels: usize,
                           bins: usize,
                           input: &[Vec<Bin>],
                           output: &mut [Vec<Bin>]| {
            TimeBlur::process(buffer,
                              freq_alpha,
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
    let mut desc = super::base_descriptor();
    desc.label = "pvoc_time_blur";
    desc.name = "pvoc time blur";
    desc.new = TimeBlur::new;
    desc.ports.push(Port {
        name: "Frequency alpha",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: None,
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc.ports.push(Port {
        name: "Amplitude alpha",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: None,
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc.ports.push(Port {
        name: "Freqency mix",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: None,
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc.ports.push(Port {
        name: "Amplitude mix",
        desc: PortDescriptor::ControlInput,
        hint: None,
        default: None,
        lower_bound: Some(0.0),
        upper_bound: Some(1.0),
    });
    desc
}
