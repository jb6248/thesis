
// goals:
// - given a chord sequence, find the prolongational branchings that relate them together
//  - find the earliest chord that it attaches to

use crate::distance::pitch_class_space::PitchClassSpace;

pub fn find_attachments(
    chords: Vec<PitchClassSpace>,
) -> Vec<(PitchClassSpace, usize)> {
    // find a maximally stable interpretation
    todo!()
}

/// Distances between sequential chords summed up.
/// This is smoother if it is minimized.
pub fn get_total_interchordal_distances(chords: &Vec<PitchClassSpace>, initial: &PitchClassSpace) -> usize {
    // for each chord, find the distance to the previous chord, and sum them up
    let mut total = 0;
    let mut prev = initial;
    for chord in chords {
        total += prev.total_distance(&chord);
        prev = chord;
    }
    total
}

/// The distance of the most distant chord from the initial chord.
/// This travels more if it is maximized.
pub fn get_maximum_distance(chords: &Vec<PitchClassSpace>, initial: &PitchClassSpace) -> usize {
    chords.iter()
        .map(|chord| initial.total_distance(chord))
        .max()
        .unwrap_or(0)
}