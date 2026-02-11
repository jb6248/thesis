// Demonstrating volume changes for dynamics
// ::v=volume sets the volume (0-100)

start Dynamics

// Soft beginning (piano)
Soft = ::v=30 :4c<1/4> :4d<1/4> :4e<1/4> :4f<1/4>

// Medium volume (mezzo-forte)
Medium = ::v=60 :4g<1/4> :4a<1/4> :4b<1/4> :5c<1/4>

// Loud ending (forte)
Loud = ::v=90 :5c<1/2> :4g<1/2>

Dynamics = Soft Medium Loud
