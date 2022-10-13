/// A message Type enum is supposed to cover all the popular message types.
/// Although this type is not that useful for now, but later it would be. In
/// Version 2.0 we'll differ a Propose message (that might contain some query)
/// from a Ping (that might be a wrapper around a Header), or perhaps from a
/// vote (that contains a body with a boolean).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Acknoweldge,
    Benchmark,
    Close,
    Commit,
    Ping,
    PreCommit,
    Propose,
    Vote,
}
