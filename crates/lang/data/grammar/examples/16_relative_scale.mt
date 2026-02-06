// Building a scale using only relative pitch movements
// Start with an absolute note, then use + to move up

start TwoScales

TwoScales = RelativeScale { RelativeScale }
RelativeScale = .<1/4> ++ .<1/4> ++ .<1/4> + .<1/4> ++ .<1/4> ++ .<1/4> ++ .<1/4> + .<1/4>

