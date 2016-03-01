use pvoc::{PhaseVocoder, Bin};
use ladspa::{PluginDescriptor, PortDescriptor, Port, Plugin, PortConnection};
use super::{PVocPlugin, PVocDescriptor};

plugin!(Centroid);

struct Centroid;

impl PVocPlugin for Centroid {
    fn descriptor() -> PVocDescriptor {
        PVocDescriptor {
            name: "pvoc centroid",
            channels: 1,
            ports: vec![],
        }
    }
    fn new(_: usize, _: f64, _: usize, _: usize) -> Centroid {
        Centroid
    }
    fn process(&mut self,
               _: &[f64],
               sample_rate: f64,
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
