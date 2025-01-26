use const_format::concatcp;



const NUMBER: &str = "5.1";

pub const MODE: &str = "dev";
// pub const MODE = "release";


pub const VERSION: &str = concatcp!(NUMBER, "-", MODE);
