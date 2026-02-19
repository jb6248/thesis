// Stepwise melodic motion using relative pitch
// Demonstrates smooth ascending and descending passages

start StepwiseMotion

// Ascending stepwise (C major scale intervals)
Ascend = + + .<1/4> + + .<1/4> + .<1/4> + + .<1/4> + + .<1/4> ++ .<1/4> + .<1/4>

// Descending stepwise
Descend = - .<1/4> - - .<1/4> - - .<1/4> -- .<1/4> - .<1/4> - - .<1/4> - - .<1/2>

StepwiseMotion = .<1/4> Ascend Descend
