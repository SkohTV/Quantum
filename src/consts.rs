use const_format::concatcp;



const MAJOR: u8 = 5;
const MINOR: u8 = 2;

pub const MODE: &str = "dev";
// pub const MODE: &str = "release";


pub const VERSION: &str = concatcp!(MAJOR, ".", MINOR, "-", MODE);
