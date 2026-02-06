// Pentatonic scale pattern using relative movements
// Pentatonic uses intervals: 2-2-3-2-3 semitones

start Pentatonic

// C pentatonic: C D E G A (C)
CPentatonic = .<1/4> + + .<1/4> + + .<1/4> + + + .<1/4> + + .<1/4> + + + .<1/4>

// Descending
CPentatonicDown = - - - .<1/4> - - .<1/4> - - - .<1/4> - - .<1/4> - - .<1/4>

Pentatonic = :4c CPentatonic CPentatonicDown
