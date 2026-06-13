# ternary-tempo

Tempo and beat detection for **ternary rhythm sequences**. Estimates BPM from onset periodicity, measures syncopation deviation, quantifies swing timing, scores beat-grid alignment, and generates rhythmic patterns in various styles from ternary-valued sequences.

## Why It Matters

Ternary rhythm sequences (values in {-1, 0, +1}) encode more musical information than binary onset grids. A `-1` (off-beat) is semantically distinct from a `0` (rest): it signals intentional anti-accent, which is critical for groove analysis. This crate extracts musically meaningful descriptors:

| Metric | Range | Meaning |
|--------|-------|---------|
| BPM | [0, ∞) | Tempo estimate |
| Syncopation | [0, 1] | Off-beat emphasis fraction |
| Swing | [0, 1] | Long-short asymmetry |
| Alignment | [0, 1] | On-grid accuracy |

## How It Works

### BPM Estimation

Beat positions are all indices where `sequence[i] ≠ 0`. The average inter-onset interval (IOI) gives the tempo:

```
beats = { i : seq[i] ≠ 0 }
intervals = [beats[k+1] - beats[k] for k in range(len(beats)-1)]
avg_ioi = mean(intervals)
BPM = 60 · ticks_per_second / avg_ioi
```

**Complexity:** O(N) — single pass for onset detection, one pass for interval computation.

### Syncopation

Measures deviation from a regular beat grid of period `beat_interval`:

```
for each onset at position i:
    dev = min(i mod interval, interval - i mod interval) / interval
syncopation = mean(deviations)
```

`syncopation = 0` means all onsets land exactly on grid positions. Higher values indicate increasing displacement toward off-beats.

**Complexity:** O(N).

### Swing Factor

Compares signal strength on even vs. odd beat groups:

```
even_strength = Σ |seq[i]|  for i mod (2·interval) < interval
odd_strength  = Σ |seq[i]|  for i mod (2·interval) ≥ interval
swing = |odd - even| / (odd + even)
```

`swing = 0` is perfectly straight; `swing → 1` is maximum asymmetry (full triplet feel).

### Beat Alignment

Fraction of onsets that fall on the beat grid:

```
interval = ⌊60 · ticks_per_second / BPM⌋
aligned = |{ i : seq[i] ≠ 0 ∧ i mod interval = 0 }|
total_active = |{ i : seq[i] ≠ 0 }|
alignment = aligned / total_active
```

**Complexity:** O(N).

### Pattern Generation

Generates a ternary rhythm from BPM and style:

| Style | Pattern |
|-------|---------|
| `FourOnFloor` | Every beat = +1 |
| `Syncopated` | Even beats = +1, odd beats = -1 |
| `Waltz` | Beat 0 = +1, beat 2 = -1, beat 1 = 0 |
| `Swing` | Even beats = +1, odd beats = -1 with shifted +1 |

Output length: `interval · 4 · measures` ticks.

## Quick Start

```rust
use ternary_tempo::{estimate_bpm, syncopation, swing_factor, generate_pattern, RhythmStyle};

let sequence: Vec<i8> = vec![1, 0, 0, 1, 0, 0, 1, 0];  // every 3rd tick
let bpm = estimate_bpm(&sequence, ticks_per_second: 100.0);
assert!(bpm > 0.0);

let sync = syncopation(&sequence, beat_interval: 3);
let swing = swing_factor(&sequence, beat_interval: 3);

let pattern = generate_pattern(120.0, 100.0, 2, RhythmStyle::FourOnFloor);
assert!(!pattern.is_empty());
```

## API

| Function | Description |
|----------|-------------|
| `estimate_bpm(seq, ticks_per_sec)` | BPM from IOI analysis |
| `syncopation(seq, beat_interval)` | Off-grid deviation [0, 1] |
| `swing_factor(seq, beat_interval)` | Even/odd asymmetry [0, 1] |
| `beat_alignment(seq, bpm, ticks_per_sec)` | On-grid fraction [0, 1] |
| `generate_pattern(bpm, ticks_per_sec, measures, style)` | Style-based generator |

## Architecture Notes

The **γ + η = C** invariant: *generation* (γ) is the pattern generation process producing ternary sequences, *entropy* (η) is the syncopation metric (higher syncopation = higher rhythmic entropy = more information content), and *conservation* (C) is the beat-alignment invariant — the total number of onsets is preserved across analysis metrics, and the BPM estimation provides a conserved "time budget" that all other metrics normalize against.

## References

- **BPM estimation:** Davies, M. & Plumbley, M. "Context-Dependent Beat Tracking" (2004)
- **Syncopation measurement:** Sioros, G. & Guedes, C. "Complexity Driven Rhythm" (2011)
- **Swing ratio:** Friberg, A. & Sundström, A. "Swing Ratios in Jazz" (1997)
- **Ternary time signatures:** Toussaint, G. *The Geometry of Musical Rhythm* (2013)

## License

MIT
