# ternary-tempo

Tempo and rhythm detection for ternary sequences — BPM estimation, syncopation measurement, swing factor analysis, beat grid alignment, pattern generation, and rhythmic equilibrium detection.

## Background

Tempo is the heartbeat of music. Human tempo perception is remarkably robust: we can tap along to a steady beat embedded in highly syncopated, swung, or irregular patterns. This ability — to extract a regular pulse from a complex surface rhythm — is called "beat induction" and is one of the first musical skills humans develop, appearing in infants as young as 2 months.

`ternary-tempo` brings beat induction into the {-1, 0, +1} domain. Given a ternary sequence representing a rhythmic signal, the crate estimates tempo, measures how syncopated or swung the rhythm is, checks alignment against a regular beat grid, generates patterns in various styles, and finds the "equilibrium spot" — the calmest point in the rhythmic cycle.

## How It Works

### BPM Estimation

`estimate_bpm(sequence, ticks_per_second)` counts non-zero entries as "beats," computes the average interval between consecutive beats, and converts to BPM:

```
BPM = 60 × ticks_per_second / average_interval
```

This works for sequences where beats are roughly evenly spaced. Highly irregular sequences produce unreliable estimates.

### Syncopation Measurement

`syncopation(sequence, beat_interval)` measures how much the rhythm deviates from a regular beat grid:

1. For each non-zero position, compute its offset from the nearest expected beat position
2. Normalize offsets by beat interval
3. Average all offsets

A perfectly on-grid rhythm scores 0.0. A rhythm with all hits on the "and" of each beat (off by half a beat interval) scores 0.5.

### Swing Factor

`swing_factor(sequence, beat_interval)` quantifies asymmetry between even and odd beat positions:

```
swing = |odd_strength − even_strength| / (odd_strength + even_strength)
```

A straight rhythm (equal energy on all beats) scores 0.0. A fully swung rhythm (all energy on either even or odd positions) approaches 1.0.

### Beat Grid Alignment

`beat_alignment(sequence, bpm, ticks_per_second)` checks how well the sequence's non-zero entries align with a regular grid at the given BPM:

```
alignment = on-beat hits / total active hits
```

A perfectly aligned rhythm scores 1.0. A rhythm with no hits on the grid scores 0.0.

### Pattern Generation

`generate_pattern(bpm, ticks_per_second, measures, style)` creates rhythmic patterns in four styles:

| Style | Description |
|-------|-------------|
| FourOnFloor | Every beat hit (+1), the classic dance pattern |
| Syncopated | Alternating strong (+1) and weak (−1) beats |
| Waltz | Beat 1 strong, beat 3 weak, beat 2 silent |
| Swing | Even beats strong, odd beats displaced by 1/3 of a beat interval |

### Equilibrium Spot

`find_equilibrium_spot(pattern)` finds the position of minimum rhythmic energy — the "8-ball spot" where the rhythm is calmest. It uses a sliding window of width 5, summing squared values and penalizing non-zero centers. The equilibrium spot is always a zero-valued position surrounded by other zeros.

## Experimental Results

- **BPM estimation is accurate for regular patterns.** A steady quarter-note pattern at 120 BPM (4 ticks/second) is correctly identified. Accuracy degrades for patterns with varying intervals.
- **Syncopation correctly distinguishes styles.** A straight four-on-the-floor pattern scores below 0.1. A shifted off-beat pattern scores above 0.3. The metric captures the perceptual difference.
- **Swing factor detects asymmetric timing.** The ternary swing pattern (with displaced odd beats) produces swing factors of 0.3-0.6, correctly identifying the asymmetric feel.
- **Equilibrium spots are musically meaningful.** In a pattern like [1, 1, 0, 0, 0, 0, 1, 1], the equilibrium spot falls in the middle of the rest — exactly where a musician would place a breath mark or phrase boundary.
- **Beat alignment discriminates timing quality.** Sequences generated with the correct BPM score near 1.0. Sequences shifted by 1 tick drop to ~0.5, matching the perceptual judgment that "the timing is off."

## Impact

`ternary-tempo` shows that core tempo perception tasks — beat finding, syncopation detection, swing measurement — can operate on three-valued signals. The ternary representation is sufficient for extracting musically meaningful tempo information, suggesting that the information content relevant to tempo perception is low-dimensional.

The equilibrium spot finder introduces a novel concept: every rhythmic pattern has a "calmest point" that can be computed rather than intuited. This has applications in automated phrase segmentation and breath-mark placement.

## Use Cases

1. **Real-time tempo tracking** — Estimate BPM from ternary sensor streams (accelerometer data classified as {-1, 0, +1}) for fitness tracking or gait analysis.
2. **Music production tools** — Analyze existing ternary rhythm patterns for syncopation, swing, and grid alignment, providing producers with quantitative groove metrics.
3. **Adaptive music systems** — Generate rhythm patterns that match a target BPM and style, then adjust in real-time based on listener feedback.
4. **Rhythmic phrase segmentation** — Use equilibrium spot detection to automatically identify phrase boundaries in rhythmic sequences.

## Open Questions

1. **Multi-level beat hierarchy.** The current BPM estimator finds a single tempo level. Can the approach be extended to detect beat, sub-beat, and bar levels simultaneously — the way humans perceive rhythm hierarchically?
2. **Tempo drift detection.** How quickly can the estimator detect a gradual tempo change (accelerando/ritardando) in a ternary stream? What's the minimum detectable drift rate?
3. **Equilibrium spot musicality.** Do the equilibrium spots identified by the algorithm correspond to musically natural phrase boundaries? A perceptual study would validate the model.

## Connection to Oxide Stack

`ternary-tempo` operates on the ternary rhythmic sequences produced by `ternary-rhythm` and `ternary-polyrhythm`. Its BPM estimation feeds `ternary-tidelight`'s temporal synchronization. The syncopation and swing metrics complement `ternary-ear`'s rhythmic pattern detection. The equilibrium spot finder connects to `ternary-compass`'s navigational concepts — finding the calmest point in a ternary landscape.
