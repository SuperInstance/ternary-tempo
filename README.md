# ternary-tempo

**BPM estimation, swing detection, and groove analysis. How fast is the heartbeat?**

Tempo is the pulse. Everything else — melody, harmony, timbre — rides on top of the tempo. And tempo isn't just "how many beats per minute." It's swing (are the off-beats late?), groove (is the rhythm locked to the beat or fighting it?), and feel (does it rush or drag?). Two drummers playing the same BPM can feel completely different because of micro-timing.

This crate estimates tempo from ternary signals, detects swing, finds the equilibrium point (the calmest spot in a rhythm pattern), and measures groove alignment.

## What's Inside

- **`estimate_bpm(signal, sample_rate)`** — estimate BPM from a ternary rhythm signal
- **`detect_swing(signal)`** — measure swing ratio. 1.0 = straight, 1.5 = light swing, 2.0 = full triplet
- **`find_equilibrium_spot(pattern)`** — find the calmest position in a pattern (lowest energy neighborhood)
- **`groove_alignment(signal, beat_interval)`** — how well does the signal align with the beat grid?
- **`generate_four_on_floor(bpm, bars)`** — the universal dance beat (four quarter notes per bar)
- **`generate_waltz(bpm, bars)`** — three-quarter time (1-2-3, 1-2-3)
- **`generate_syncopated(bpm, bars)`** — off-beat patterns (jazz, funk, reggae)
- **`equilibrium_in_zeros(pattern)`** — the equilibrium point should be at a zero value

## Quick Example

```rust
use ternary_tempo::*;

// Generate a four-on-the-floor at 120 BPM
let beat = generate_four_on_floor(120.0, 4);
// Four bars of quarter-note pulses

// Detect the BPM
let bpm = estimate_bpm(&beat, 44100.0);
println!("Detected: {:.0} BPM", bpm);

// Generate a swing pattern
let swung = generate_syncopated(100.0, 2);
let swing_ratio = detect_swing(&swung);
println!("Swing: {:.2} (1.0 = straight)", swing_ratio);

// Find the calm spot in a pattern
let pattern = vec![1, 1, 0, 0, 0, 0, 1, 1];
let eq = find_equilibrium_spot(&pattern);
println!("Equilibrium at position {} (value: {})", eq, pattern[eq]);
// Should be at one of the zeros
```

## The Deeper Truth

**Tempo is a perception, not a measurement.** Two signals at the same BPM can feel different speeds depending on where the accents fall. A pattern with accents on beats 1 and 3 (four-on-the-floor) feels slower than the same pattern with accents on 1, 2, 3, 4 (double-time feel). The tempo is in the *pattern*, not just the frequency.

The equilibrium spot is the most musical function: it finds the "rest" in a rhythm — the place where the energy is lowest. In a drum pattern, that's where you'd put a fill or a break. It's the musical equivalent of a negative space in visual art — the silence that gives the sound its shape.

**Use cases:**
- **BPM detection** — estimate tempo from any ternary rhythm
- **Swing analysis** — measure how swung a pattern is
- **Rhythm generation** — generate standard patterns (four-on-floor, waltz, syncopated)
- **Groove quantization** — align loose timing to a grid
- **Music information retrieval** — classify music by tempo and feel

## See Also

- **ternary-rhythm** — the patterns that tempo measures
- **ternary-polyrhythm** — multiple tempos simultaneously
- **ternary-phase** — phase alignment at the tempo level
- **ternary-jam** — tempo emerges from jam dynamics
- **ternary-fib** — period-8 as the natural ternary tempo

## Install

```bash
cargo add ternary-tempo
```

## License

MIT
