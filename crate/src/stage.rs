use crate::tile::Tile;


pub type Stage<const X : usize, const Y : usize> = [[Tile; X]; Y];


