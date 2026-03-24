// Let's assume we are in common (4/4) time and harmony changes every 2 beats (half measure)
// Notes are quarter notes by default
// We will assume a tonal center of C

start composition

// This is what the I/I pitch class space looks like:
// a. 0
// b. 0             7
// c. 0       4     7
// d. 0   2   4 5   7   9   b
// e. 0 1 2 3 4 5 6 7 8 9 a b

// both of these grow by 2 beats every iteration, so as long as it is rendered the same depth on both
// sides, the lengths will match.
composition = { generating-harmony | [x12][+] generating-melody }

// this grows by 2 beats every iteration.
generating-harmony = I/I-harmony generating-harmony

// this just has the ones I defined here but this could be automatically generated for all of them
random-harmony = { I/I-harmony | IV/I-harmony | I/V-harmony }

// These can be generated automatically. All the chords with regions can be enumerated using a cartesian product
// of the symbols listed below:
// I, i, II, ii, ..., VII, vii
// Harmonies can be generated based on the pitch class spaces and their triadic levels.
I/I-harmony = { :c3<1/2> | :e3<1/2> | :g3<1/2> }
IV/I-harmony = { :c3<1/2> | :f3<1/2> | :a3<1/2> }
// ... more chords in the same   region
I/V-harmony = { :g2<1/2> | :d3<1/2> | :b3<1/2> }
// ... more chords in different regions

// Go up or down by semitone, whole step, minor third, major third, perfect fourth, perfect fifth
// Melodies that move around too much are not very pleasurable, according to Jazz paper.
random-interval = { [x7][-] | ----- | ---- | --- | -- | - | + | ++ | +++ | ++++ | +++++ | [x7][+] }

// This just wanders around randomly, with some variation in rhythm.
// This grows by 2 beats every iteration.
generating-melody = { .<1/2> | .<1/4> random-interval .<1/4> | .<3/8> random-interval .<1/8> } random-interval generating-melody