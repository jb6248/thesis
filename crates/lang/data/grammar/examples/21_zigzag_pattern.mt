// Zigzag melodic pattern using relative movements
// Creates an interesting rhythmic and melodic texture

start Zigzag

// Small zigzag
SmallZig = + .<1/8> - - .<1/8> + .<1/8>

// Large zigzag
LargeZig = + + + .<1/8> - - - - .<1/8> + + .<1/8>

Zigzag = :4e [x4][SmallZig] [x2][LargeZig]
