#![feature(type_ascription)]
extern crate ladspa;
extern crate pvoc;

use ladspa::PluginDescriptor;

mod binflipper;
mod pitchshifter;
mod centroid;

#[no_mangle]
pub fn get_ladspa_descriptor(index: u64) -> Option<PluginDescriptor> {
    match index {
        0 => Some(binflipper::get_descriptor()),
        1 => Some(pitchshifter::get_descriptor()),
        2 => Some(centroid::get_descriptor()),
        _ => None,
    }
}