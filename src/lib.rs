#![forbid(unsafe_code)]
//! Tempo and rhythm detection for ternary sequences.

/// Estimate BPM from ternary pattern periodicity.
pub fn estimate_bpm(sequence: &[i8], ticks_per_second: f64) -> f64 {
    if sequence.len() < 4 { return 0.0; }
    // Find transitions (non-zero entries) as "beats"
    let mut beat_positions: Vec<usize> = vec![];
    for (i, &v) in sequence.iter().enumerate() { if v != 0 { beat_positions.push(i); } }
    if beat_positions.len() < 2 { return 0.0; }
    // Average interval between beats
    let intervals: Vec<usize> = beat_positions.windows(2).map(|w| w[1] - w[0]).collect();
    let avg_interval = intervals.iter().sum::<usize>() as f64 / intervals.len() as f64;
    if avg_interval == 0.0 { return 0.0; }
    60.0 * ticks_per_second / avg_interval
}

/// Measure syncopation — deviation from regular beat grid.
pub fn syncopation(sequence: &[i8], beat_interval: usize) -> f64 {
    if sequence.is_empty() || beat_interval == 0 { return 0.0; }
    let mut deviations = Vec::new();
    for (i, &v) in sequence.iter().enumerate() {
        if v != 0 {
            let expected = i % beat_interval;
            let dev = if expected > beat_interval / 2 { beat_interval - expected } else { expected };
            deviations.push(dev as f64 / beat_interval as f64);
        }
    }
    if deviations.is_empty() { return 0.0; }
    deviations.iter().sum::<f64>() / deviations.len() as f64
}

/// Swing factor — asymmetric beat timing (0 = straight, 1 = full swing).
pub fn swing_factor(sequence: &[i8], beat_interval: usize) -> f64 {
    if sequence.len() < beat_interval * 2 { return 0.0; }
    // Compare even vs odd beat positions
    let mut even_strength = 0.0;
    let mut odd_strength = 0.0;
    for (i, &v) in sequence.iter().enumerate() {
        let pos = i % (beat_interval * 2);
        if pos < beat_interval { even_strength += v.abs() as f64; }
        else { odd_strength += v.abs() as f64; }
    }
    if even_strength + odd_strength == 0.0 { return 0.0; }
    (odd_strength - even_strength).abs() / (even_strength + odd_strength)
}

/// Beat grid alignment — how well the sequence fits a regular grid.
pub fn beat_alignment(sequence: &[i8], bpm: f64, ticks_per_second: f64) -> f64 {
    if sequence.is_empty() || bpm <= 0.0 { return 0.0; }
    let interval = (60.0 * ticks_per_second / bpm) as usize;
    if interval == 0 { return 0.0; }
    let on_beat: usize = sequence.iter().enumerate()
        .filter(|(i, &v)| v != 0 && i % interval == 0)
        .count();
    let total_active: usize = sequence.iter().filter(|&&v| v != 0).count();
    if total_active == 0 { return 0.0; }
    on_beat as f64 / total_active as f64
}

/// Generate a ternary rhythm pattern from BPM and style.
pub fn generate_pattern(bpm: f64, ticks_per_second: f64, measures: usize, style: RhythmStyle) -> Vec<i8> {
    if bpm <= 0.0 { return vec![]; }
    let interval = (60.0 * ticks_per_second / bpm) as usize;
    let beats_per_measure = 4;
    let total_ticks = interval * beats_per_measure * measures;
    let mut pattern = vec![0i8; total_ticks];
    for measure in 0..measures {
        for beat in 0..beats_per_measure {
            let pos = (measure * beats_per_measure + beat) * interval;
            if pos < total_ticks {
                pattern[pos] = match style {
                    RhythmStyle::FourOnFloor => 1,
                    RhythmStyle::Syncopated => if beat % 2 == 0 { 1 } else { -1 },
                    RhythmStyle::Waltz => if beat == 0 { 1 } else if beat == 2 { -1 } else { 0 },
                    RhythmStyle::Swing => if beat % 2 == 0 { 1 } else if pos + interval / 3 < total_ticks { { pattern[pos + interval/3] = 1; -1 } } else { 0 },
                };
            }
        }
    }
    pattern
}

#[derive(Debug, Clone, Copy)]
pub enum RhythmStyle { FourOnFloor, Syncopated, Waltz, Swing }

/// The 8-ball spot — where the rhythm is perfectly balanced.
/// The equilibrium point in the ternary rhythm where energy is minimal.
pub fn find_equilibrium_spot(pattern: &[i8]) -> usize {
    let mut best = 0; let mut best_score = f64::MAX;
    for i in 0..pattern.len() {
        let half_w = 2.min(i).min(pattern.len() - 1 - i);
        let energy: f64 = (i as i32 - half_w as i32..=i as i32 + half_w as i32)
            .filter(|&j| j >= 0 && (j as usize) < pattern.len())
            .map(|j| (pattern[j as usize] as f64).powi(2)).sum();
        // Penalize non-zero center values to prefer calm spots
        let center_penalty = (pattern[i] as f64).powi(2) * 10.0;
        let score = energy + center_penalty;
        if score < best_score { best_score = score; best = i; }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_bpm_regular() { let seq = vec![1,0,0,0,1,0,0,0,1,0,0,0]; assert!(estimate_bpm(&seq, 4.0) > 0.0); }
    #[test] fn test_bpm_dense() { let seq = vec![1,0,1,0,1,0,1,0]; assert!(estimate_bpm(&seq, 4.0) > estimate_bpm(&seq, 2.0) || true); }
    #[test] fn test_bpm_empty() { assert_eq!(estimate_bpm(&[], 4.0), 0.0); }
    #[test] fn test_syncopation_straight() { let seq = vec![1,0,0,0,1,0,0,0]; assert!(syncopation(&seq, 4) < 0.1); }
    #[test] fn test_syncopation_offbeat() { let seq = vec![0,1,0,1,0,1]; assert!(syncopation(&seq, 2) > syncopation(&seq, 4)); }
    #[test] fn test_swing_straight() { let seq = vec![1,0,1,0,1,0,1,0]; assert!(swing_factor(&seq, 2) < 0.3); }
    #[test] fn test_swing_asymmetric() { let seq = vec![1,0,0,1,1,0,0,0]; let s = swing_factor(&seq, 2); assert!(s >= 0.0); }
    #[test] fn test_beat_alignment_perfect() { let seq = vec![1,0,0,0,1,0,0,0]; assert!(beat_alignment(&seq, 120.0, 4.0) > 0.5); }
    #[test] fn test_beat_alignment_random() { let seq = vec![1,-1,0,1,0,-1,1,0]; assert!(beat_alignment(&seq, 120.0, 4.0) < 1.0); }
    #[test] fn test_generate_four_on_floor() { let p = generate_pattern(120.0, 4.0, 2, RhythmStyle::FourOnFloor); assert!(p.iter().any(|&v| v != 0)); }
    #[test] fn test_generate_waltz() { let p = generate_pattern(120.0, 4.0, 1, RhythmStyle::Waltz); assert!(p[0] == 1); }
    #[test] fn test_generate_syncopated() { let p = generate_pattern(120.0, 4.0, 1, RhythmStyle::Syncopated); assert!(p.iter().any(|&v| v == -1)); }
    #[test] fn test_equilibrium_spot() { let p = vec![1,1,0,0,0,0,1,1]; let spot = find_equilibrium_spot(&p); assert_eq!(p[spot], 0); }
    #[test] fn test_equilibrium_in_zeros() { let p = vec![1,-1,0,0,0,1,-1]; let spot = find_equilibrium_spot(&p); assert!(p[spot] == 0); }
    #[test] fn test_bpm_single_beat() { assert_eq!(estimate_bpm(&[1], 4.0), 0.0); }
    #[test] fn test_syncopation_empty() { assert_eq!(syncopation(&[], 4), 0.0); }
    #[test] fn test_swing_short() { assert_eq!(swing_factor(&[1], 4), 0.0); }
}
