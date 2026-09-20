#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyKind {
  Normal,
  #[allow(dead_code)]
  Applicative,
  #[allow(dead_code)]
  CallByValue,
}
