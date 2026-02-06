// Walking bass line using relative pitch movements
// Demonstrates how to create a bass line that "walks" up and down

start WalkingBass

// Walk up from root
WalkUp = .<1/4> + + .<1/4> + + .<1/4> + .<1/4>

// Walk down to fifth
WalkDown = - - .<1/4> - - .<1/4> - .<1/4> - - .<1/4>

// Back to root
ReturnToRoot = + + .<1/4> + + .<1/4> + + .<1/4> + .<1/2>

WalkingBass = ::i=Bass :3c WalkUp WalkDown ReturnToRoot
