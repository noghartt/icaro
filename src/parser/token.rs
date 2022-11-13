#[derive(Debug, PartialEq)]
pub enum Token {
  LPar,        // (
  RPar,        // )

  LInt(u64),

  OpPlus,      // +

  EOF          // End of file
}
