#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LPar,        // (
    RPar,        // )
    Int(usize),  // [0-9]
    Str(String), // "[^"]*"
    Id(String),  // [^ ()]
    Err,         // Sentinel err token
    EOF,         // End of file
}
