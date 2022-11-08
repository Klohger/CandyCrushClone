/*
pub struct Tile<'a> {
        previousTile : &'a Tile<'a>,
        nextTile : &'a Tile<'a>,
        switchable : bool,
    }
    */


pub struct Tile {
    entrancePosition : Option<(usize,usize)>,
    look : usize,
    previous : Option<(usize,usize)>,
    next : Option<(usize,usize)>,
}