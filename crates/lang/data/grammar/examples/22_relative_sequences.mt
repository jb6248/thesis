// Musical sequences using relative pitch and transforms
// A sequence is a melodic pattern repeated at different pitch levels

start Sequence

// Basic motif using relative pitch
Motif = .<1/8> + .<1/8> + .<1/8> - - - .<1/4>

// Repeat the motif starting from different pitches
Sequence = :4c Motif + + Motif + + Motif - - - - Motif
