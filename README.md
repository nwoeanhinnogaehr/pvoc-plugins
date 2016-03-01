# pvoc-plugins
A collection of phase vocoder based LADSPA plugins.
Compile with `cargo build --release`, throw `target/release/libpvoc_plugins.so` in your LADSPA path, and you're good to go!

## The plugins
* Bins log2: the number of frequency bins used for the phase vocoder. Few will likely be low quality and many will blur the audio through time. Somewhere between 6 and 13 is usually what you want.
* Time divs: the number of overlapping frames to use. Powers of two between 4 and 32 are good choices.

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

### amplitude scaled delay
Each bin is delayed by an amount relative to it's amplitude. Delay is measured in frames that are bins/time-div/sample-rate seconds long.
* Delay: amount of time to delay by
* Max delay: delay buffer size
* Frequency/amplitude mix: mixer for delayed/original signal
* Frequency/amplitude feedback: multiplier for previously read events - at 1, samples will remain in the buffer until they are overwritten, possibly looping after the max delay.

### gate
Filter out loud/quiet sounds
* Gate: don't let sounds through that are quieter than this threshold
* Duck: don't let sounds through that are louder than this threshold

### slope filter
Filter out sounds that are changing in frequency or amplitude
* Freq/Amp min/max: thresholds for filter to activate

### repeater
Capture a sound then repeat it indefinitely
* Length: The length of the section to repeat, in frames
* Hold: Mixer for input signal/signal from loop buffer

## Contributing
Contributions (new plugins, fixes, etc) are welcome! To make sure everything goes smoothly, please ensure your code is formatted with `rustfmt`, produces no warnings and follows existing conventions to the extent possible.
