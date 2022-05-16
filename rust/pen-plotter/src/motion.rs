use crate::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub path: Path,
    pub start: Instant,
}
