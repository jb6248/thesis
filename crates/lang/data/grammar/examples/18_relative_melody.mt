// Creating a simple melody using only relative movements
// Each non-terminal defines a melodic gesture

start Melody

// Up by major third (4 semitones)
UpThird = + + + +

// Down by perfect fifth (7 semitones)
DownFifth = - - - - - - -

// Small oscillation
Trill = + - + -

Melody = :4c .<1/4> UpThird .<1/4> DownFifth .<1/4> [x4][Trill .<1/16>]
