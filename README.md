# ternary-tempo

**Tempo and rhythm detection for ternary sequences.** BPM estimation, syncopation, swing timing — the metronome for ternary conversations.

## Why This Exists

Multi-agent ternary conversations have a natural rhythm. In `ternary-tenforward`, we discovered that the Fibonacci period 8 (Pisano period mod 3) governs when agents tunnel out of reflection. But that's the micro-rhythm. What about the macro-rhythm?

When agents in a population alternate between active and passive states, there's a detectable tempo. An agreeing population pulses at one rate; a divided population has a different feel entirely. This crate measures that tempo, quantifies how off-beat the agents are (syncopation), and detects asymmetric timing patterns (swing).

Without tempo detection, you can't know when to trigger transitions, when to apply effects, or when the conversation is speeding up or slowing down. Tempo is the foundation of everything else in the DJ metaphor stack.

## The Physics Behind It

### BPM from Ternary Sequences

Ternary sequences encode agent states over time: -1 (contrarian), 0 (reflecting), +1 (agreeing). "Beats" are the non-zero entries — moments when an agent takes a stance. BPM estimation works by:

1. Finding all beat positions (non-zero values)
2. Computing intervals between consecutive beats
3. Averaging those intervals
4. Converting to beats-per-minute: `BPM = 60 × ticks_per_second / avg_interval`

This is analogous to onset detection in audio signal processing, except the "signal" is a discrete ternary stream rather than a continuous waveform.

### Syncopation

Syncopation measures deviation from a regular beat grid. For each active position, we compute how far it falls from the expected grid alignment. High syncopation means agents are speaking at unpredictable intervals — the conversation has a lot of interjections and interruptions. Low syncopation means a polite, turn-taking pattern.

In the RPS experiments, high syncopation correlated with rapid dominance cycling. When syncopation was low, one agent tended to dominate for longer stretches.

### Swing

Swing is asymmetric beat timing — the "long-short" pattern familiar from jazz. We measure it by comparing the aggregate energy on even vs. odd beat subdivisions. Perfect swing (value 1.0) means all the activity is on one subdivision; 0.0 means perfectly even.

### The Equilibrium Spot

`find_equilibrium_spot` locates the 8-ball position — where the rhythm has minimum energy. This is the natural rest point, useful for placing transitions or letting the conversation breathe. It connects to the spindle concept in `ternary-crossfader`.

### Rhythm Styles

Four preset patterns map to different conversation dynamics:

- **FourOnFloor** — every beat active. A lively, engaged conversation where everyone contributes regularly.
- **Syncopated** — alternating strong/weak beats. A debate with call-and-response patterns.
- **Waltz** — 3/4 time with emphasis on the first beat. A conversation with a clear leader and respondents.
- **Swing** — delayed offbeats. A relaxed, informal exchange with natural timing.

## Key Types and Functions

```rust
/// Rhythm style presets for pattern generation.
pub enum RhythmStyle { FourOnFloor, Syncopated, Waltz, Swing }

/// Estimate BPM from ternary pattern periodicity.
pub fn estimate_bpm(sequence: &[i8], ticks_per_second: f64) -> f64

/// Measure syncopation — deviation from regular beat grid.
pub fn syncopation(sequence: &[i8], beat_interval: usize) -> f64

/// Swing factor — asymmetric beat timing (0 = straight, 1 = full swing).
pub fn swing_factor(sequence: &[i8], beat_interval: usize) -> f64

/// Beat grid alignment — how well the sequence fits a regular grid.
pub fn beat_alignment(sequence: &[i8], bpm: f64, ticks_per_second: f64) -> f64

/// Generate a ternary rhythm pattern from BPM and style.
pub fn generate_pattern(bpm: f64, ticks_per_second: f64, measures: usize, style: RhythmStyle) -> Vec<i8>

/// The 8-ball spot — equilibrium point where rhythm energy is minimal.
pub fn find_equilibrium_spot(pattern: &[i8]) -> usize
```

## Usage

```rust
use ternary_tempo::{estimate_bpm, syncopation, swing_factor, beat_alignment,
                    generate_pattern, find_equilibrium_spot, RhythmStyle};

// A conversation where agents speak every 4 ticks
let conversation = vec![1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0];

let bpm = estimate_bpm(&conversation, 4.0);  // ~60 BPM
let sync = syncopation(&conversation, 4);     // low — very regular
let swing = swing_factor(&conversation, 4);   // low — no asymmetry
let alignment = beat_alignment(&conversation, 60.0, 4.0);  // high — on the grid

// Find where to breathe
let rest = find_equilibrium_spot(&conversation);

// Generate a pattern
let debate = generate_pattern(120.0, 4.0, 2, RhythmStyle::Syncopated);
// Produces: [1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, ...]

let casual = generate_pattern(90.0, 4.0, 2, RhythmStyle::Swing);
// Offbeats are delayed — natural conversation feel
```

### Tempo-Driven Crossfading

```rust
// Use tempo to drive transitions in ternary-crossfader
let bpm = estimate_bpm(&agent_history, 4.0);
if bpm > 100.0 {
    // Fast conversation — use S-curve for quick transitions
    // (pass to ternary-crossfader with FaderCurve::SCurve)
}
```

## Experiment Connections

| Measurement | What It Tells You |
|------------|-------------------|
| High BPM (>100) | Agents are actively engaged, rapid state changes |
| Low BPM (<40) | Agents are stuck in reflection, low energy |
| High syncopation (>0.5) | Unpredictable, chaotic conversation |
| Low syncopation (<0.1) | Ordered, possibly monoculture risk |
| High swing (>0.5) | Asymmetric roles — one agent leads |
| Low alignment (<0.3) | Free-form, not grid-locked |

The monoculture warning from the ten-forward experiments: if syncopation drops to near-zero and BPM stabilizes, the conversation may be locking up. This is the early warning system.

## In the Ternary Fleet

This is the **timing layer** in the DJ metaphor product stack:

- `ternary-tenforward` — produces the ternary agent streams
- **ternary-tempo** — measures the rhythm of those streams
- `ternary-crossfader` — uses tempo to time transitions
- `ternary-envelope` — uses tempo to shape ADSR profiles
- `ternary-grain` — uses tempo to set grain density and size

## License

MIT
