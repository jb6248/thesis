// Arpeggio patterns using current pitch manipulation
// Starting from a base note and building up intervals

start Arpeggio

// C major arpeggio using relative pitch movements
CMajorArp = :4c .<1/8> [T4][.<1/8>] [T3][.<1/8>] [T5][.<1/8>]

// Minor arpeggio
CMinorArp = :4c .<1/8> [T3][.<1/8>] [T4][.<1/8>] [T5][.<1/8>]

Arpeggio = [x2][CMajorArp] [x2][CMinorArp]
