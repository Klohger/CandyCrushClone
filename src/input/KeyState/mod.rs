pub type KeyState = usize;
pub const THIS_TICK: KeyState = 0b10;
pub const NOT_THIS_TICK: KeyState = 0b00;
pub const HELD: KeyState = 0b01;
pub const RELEASED: KeyState = 0b00;

pub fn held(key_state: KeyState) -> bool {
    return (key_state & HELD) == 1;
}
pub fn this_tick(key_state: KeyState) -> bool {
    return (key_state & THIS_TICK) == 2;
}
pub fn set_bit(num: usize, i: usize, x: usize) -> usize {
    return (num & !(1 << i)) | (x << i);
}
pub fn clear_bit(num: usize, i: usize) -> usize {
    return num & !(1 << i);
}
