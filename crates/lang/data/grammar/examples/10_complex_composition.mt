// A more complex composition combining multiple features
// Shows repeats, transposition, splits, and instrument changes

start Song

// Main melody with repeats
Melody = [x2][:4c<1/4> :4e<1/4> :4g<1/4> :5c<1/2>]

// Bass line (transposed down an octave)
BassLine = [T-12][Melody]

// Harmonic variation (transposed up)
Harmony = [T7][Melody]

// Drum pattern with different compression
DrumPattern = ::i=BassDrum [>>2][.<1/4> *<1/4> .<1/4> *<1/4>]

// Combine everything
Song = { ::i=Piano ::v=80 Melody | ::i=Bass ::v=70 BassLine | DrumPattern }
