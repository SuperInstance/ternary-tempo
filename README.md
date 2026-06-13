# Ternary Tempo — Rhythm Detection and Generation for Ternary Sequences

**Ternary Tempo** analyzes rhythmic patterns in ternary-valued sequences {-1, 0, +1}. It estimates BPM from beat periodicity, measures syncopation and swing, computes beat-grid alignment, and generates new patterns in various styles (four-on-floor, waltz, syncopated, swing). Designed for ternary signal analysis where periodicity matters.

## Why It Matters

Periodic patterns appear everywhere in ternary systems: agent state oscillations, GPU utilization cycles, consensus round timing. Detecting the "tempo" of these patterns — how fast they cycle, how well they align to a regular grid, whether they swing — provides diagnostic insight. A fleet oscillating at high frequency between +1 and -1 (high BPM, low syncopation) is making rapid but predictable decisions. A fleet with high syncopation is making irregular decisions, potentially indicating instability. The rhythm generation capability also enables constructing scheduled ternary patterns (like a score for fleet operations).

## How It Works

### BPM Estimation

Finds non-zero entries (beats) in the sequence and computes the average inter-beat interval:

```
BPM = 60 × ticks_per_second / avg_interval
```

O(n) for n sequence elements. Returns 0 if fewer than 2 beats are found.

### Syncopation

Measures deviation from a regular beat grid. For each beat, computes its offset from the nearest expected position (given the beat interval), normalized by the interval:

```
syncopation = mean(|offset_i| / interval) for all beats
```

Syncopation = 0 means all beats fall on the grid; syncopation = 0.5 means beats are typically halfway between grid positions. O(n).

### Swing Factor

Compares the strength of even vs. odd beat positions:

```
swing = |odd_strength - even_strength| / (odd_strength + even_strength)
```

Swing = 0 is perfectly straight (equal weight); swing → 1 is full swing (lopsided). O(n).

### Beat Alignment

Fraction of non-zero entries that fall on expected beat positions:

```
alignment = count(on_beat) / count(active)
```

O(n). High alignment means the pattern is regular; low alignment means it's irregular.

### Pattern Generation

`generate_pattern(bpm, tps, measures, style)` creates a ternary rhythm:

- **Four-on-floor**: +1 on every beat, 0 elsewhere
- **Syncopated**: Alternating +1 and -1 on beats
- **Waltz**: +1 on beat 1, -1 on beat 3, 0 on beat 2
- **Swing**: +1 on even beats, delayed +1 on odd beats

O(beats × measures) generation.

## Quick Start

```rust
use ternary_tempo::{estimate_bpm, syncopation, RhythmStyle, generate_pattern};

let pattern = generate_pattern(120.0, 100.0, 4, RhythmStyle::FourOnFloor);
let bpm = estimate_bpm(&pattern, 100.0);
let sync = syncopation(&pattern, 25);
println!("BPM: {:.0}, Syncopation: {:.2}", bpm, sync);
```

```bash
cargo add ternary-tempo
```

## API

| Type / Function | Description |
|---|---|
| `estimate_bpm(seq, ticks_per_sec)` | BPM from inter-beat intervals |
| `syncopation(seq, beat_interval)` | Grid deviation (0 = on-grid) |
| `swing_factor(seq, beat_interval)` | Asymmetry metric |
| `beat_alignment(seq, bpm, tps)` | Fraction of beats on grid |
| `generate_pattern(bpm, tps, measures, style)` | Synthesize a pattern |
| `RhythmStyle` | FourOnFloor, Syncopated, Waltz, Swing |

## Architecture Notes

Tempo analysis reveals the operational rhythm of **SuperInstance** fleets. Fast, regular tempos (high BPM, low syncopation) indicate healthy fleet operations; irregular tempos indicate instability. The γ + η = C conservation manifests in the regularity: regular patterns have high γ (predictable growth) and low η (low entropy), while irregular patterns have high η. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Temperley, David. *The Cognition of Basic Musical Structures*, MIT Press, 2001.
| Krumhansl, Carol. *Cognitive Foundations of Musical Pitch*, Oxford UP, 1990.
| Toussaint, Godfried. *The Geometry of Musical Rhythm*, CRC Press, 2013.

## License

MIT
