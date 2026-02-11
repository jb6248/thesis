// Nested transforms: combining repeats, transposition, and compression
// Demonstrates how transforms can be stacked

start NestedExample

BasePattern = :4c<1/4> :4e<1/4> :4g<1/4>

// Repeat 2 times, then transpose up 5 semitones
RepeatedTransposed = [T5][[x2][BasePattern]]

// Make it twice as fast
FastVersion = [>>2][RepeatedTransposed]

NestedExample = BasePattern FastVersion
