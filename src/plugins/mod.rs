use ladspa::PluginDescriptor;

pub use super::{PVocPlugin, PVocDescriptor};

mod binflipper;
mod pitchshifter;
mod centroid;
mod expavg;
mod modularamp;
mod formantshifter;
mod freqshifter;
mod domainxover;
mod timeblur;
mod ampdelay;
mod gate;
mod slopefilter;
mod repeater;
mod scrambler;
mod stencil;

#[no_mangle]
pub fn get_ladspa_descriptor(index: u64) -> Option<PluginDescriptor> {
    match index {
        0 => Some(binflipper::get_descriptor()),
        1 => Some(pitchshifter::get_descriptor()),
        2 => Some(centroid::get_descriptor()),
        3 => Some(expavg::get_descriptor()),
        4 => Some(modularamp::get_descriptor()),
        5 => Some(formantshifter::get_descriptor()),
        6 => Some(freqshifter::get_descriptor()),
        7 => Some(domainxover::get_descriptor()),
        8 => Some(timeblur::get_descriptor()),
        9 => Some(ampdelay::get_descriptor()),
        10 => Some(gate::get_descriptor()),
        11 => Some(slopefilter::get_descriptor()),
        12 => Some(repeater::get_descriptor()),
        13 => Some(scrambler::get_descriptor()),
        14 => Some(stencil::get_descriptor()),
        _ => None,
    }
    .map(|mut x| {
        x.unique_id += index;
        x
    })
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a * x + b * (1.0 - x)
}
