# pvoc-plugins
A collection of phase vocoder based LADSPA plugins.
Compile with `cargo build --release`, throw `target/release/libpvoc_plugins.so` in your LADSPA path, and you're good to go!

## The plugins
Every plugin has a "Bins" setting - this controls the number of frequency bins used for the phase vocoder. Few will likely be low quality and many will blur the audio through time. For optimal performance, this should be a power of two or the sum of a small number of powers of two.

### bin flipper
This linearly inverts the frequency of each bin.
* Nyquist multiplier: multiplier for the center frequencies of the bins.

### centroid
Fixes the frequency of each bin directly to the center.

### domain crossover
Modulates frequency of the bins based on their amplitude.
* Add: Ring modulation factor - intensity of frequency modulation.
* Shift: Frequency offset.
* Alpha: Exponential averaging alpha for amplitude estimate.

### exponential averaging
Modulates frequency and amplitude of bins based on exponential average of lower pitched bins.
* Frequency alpha: exponential averaging alpha for frequency
* Amplitude alpha: exponential averaging alpha for amplitude
* Frequency mix: Mixer for original/modulated frequency
* Amplitude mix: Mixer for original/modulated amplitude

### formant shifter
* Shift: Shift factor

### frequency shifter
* Shift: Shift factor

### pitch shifter
* Shift: Shift factor

### modular amplitude
Performs floating point modulus on the amplitude of each bin.
* Mod: Divisor

### time blur
Uses exponential averaging to blur amplitude and frequency across time.
* Frequency alpha: exponential averaging alpha for frequency
* Amplitude alpha: exponential averaging alpha for amplitude
* Frequency mix: Mixer for original/modulated frequency
* Amplitude mix: Mixer for original/modulated amplitude
* Amplitude high replace: Mixer for replacing blurred amplitude with current amplitude when current amplitude exceeds blurred amplitude.
* Amplitude low replace: Mixer for replacing blurred amplitude with current amplitude when blurred amplitude exceeds current amplitude.
