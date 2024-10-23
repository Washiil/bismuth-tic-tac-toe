pub const ROW1: u16 = 0b_111_000_000;
pub const ROW2: u16 = 0b_000_111_000;
pub const ROW3: u16 = 0b_000_000_111;

pub const COL1: u16 = 0b_100_100_100;
pub const COL2: u16 = 0b_010_010_010;
pub const COL3: u16 = 0b_001_001_001;

pub const DIA1: u16 = 0b_100_010_001;
pub const DIA2: u16 = 0b_001_010_100;

pub const FILLED_BOARD: u16 = 0b_111_111_111;

pub const WINNING_POSITIONS: [u16; 8] = [COL1, COL2, COL3, ROW1, ROW2, ROW3, DIA1, DIA2];

