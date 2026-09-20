#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyKind {
  Normal,
  Applicative,
  CallByValue,
}
