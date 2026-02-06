// Recursive grammar for generating musical patterns
// Non-terminals can reference each other

start Phrase

Phrase = MotifA MotifB MotifA Ending
MotifA = :4c<1/4> :4e<1/4> :4g<1/4> :4e<1/4>
MotifB = :4d<1/4> :4f<1/4> :4a<1/4> :4f<1/4>
Ending = :4c<1>
