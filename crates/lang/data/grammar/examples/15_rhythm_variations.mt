// Different rhythm patterns using various note durations
// Demonstrates whole notes (1), half notes (1/2), quarters (1/4), eighths (1/8), etc.

start RhythmStudy

// Whole notes
WholeNotes = :4c<1> :4e<1> :4g<1>

// Half notes
HalfNotes = :4c<1/2> :4d<1/2> :4e<1/2> :4f<1/2>

// Quarter notes
QuarterNotes = :4c<1/4> :4d<1/4> :4e<1/4> :4f<1/4> :4g<1/4> :4a<1/4> :4b<1/4> :5c<1/4>

// Eighth notes
EighthNotes = [>>2][QuarterNotes]

// Mixed rhythms
MixedRhythm = :4c<1/2> :4e<1/4> :4g<1/4> :5c<1>

RhythmStudy = WholeNotes HalfNotes QuarterNotes MixedRhythm
