// window
pub const WH: usize = 520;
pub const WW: usize = 520;

// board
pub const GL: f32 = 50.0; // grid length
pub const BW: f32 = GL * 8.0; // board width
pub const BH: f32 = GL * (10 + 1) as f32; // board height
pub const LINE_WIDTH: f32 = GL / 50.0;
// board target mark
pub const SP: f32 = GL / 10.0; // space
pub const EL: f32 = GL / 5.0; // edge length

// color
pub const COLOR_BOARD: (u8, u8, u8) = (201, 114, 32);
