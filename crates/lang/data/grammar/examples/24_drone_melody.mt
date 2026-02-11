// Melody over a drone using relative pitch and splits
// The drone stays constant while melody moves

start DroneWithMelody

// Drone just repeats the same note
Drone = [x8][.<1/4>]

// Melody moves around using relative pitch
Melody = .<1/4> + + .<1/4> + .<1/4> + + .<1/4> - .<1/4> - - .<1/4> - .<1/4> - - .<1/4>

DroneWithMelody = { :3g Drone | :4g Melody }
