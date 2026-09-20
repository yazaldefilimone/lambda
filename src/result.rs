#[derive(Debug, Clone, Copy)]
pub enum Failed {
  Abort,
  Recover,
  Load,
}

pub type Result<T> = std::result::Result<T, Failed>;
