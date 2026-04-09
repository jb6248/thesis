// Changing instruments with meta-controls
// ::i=piano sets instrument to piano
// ::v=80 sets volume to 80

start MultiInstrument

Piano = ::i=Piano :4c<1/4> :4d<1/4> :4e<1/4> :4f<1/4>
Bass = ::i=Bass :3c<1/2> :3g<1/2>

MultiInstrument = {Piano | Bass}
