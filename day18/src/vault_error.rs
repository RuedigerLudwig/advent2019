#![allow(dead_code)]

#[derive(Debug)]
pub enum VaultError {
    UnknownTile(char),
    ExactlyOneEntrance,
    NoPath,
}

impl std::error::Error for VaultError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VaultError::UnknownTile(_) => None,
            VaultError::ExactlyOneEntrance => None,
            VaultError::NoPath => None,
        }
    }
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VaultError::UnknownTile(tile) => {
                write!(f, "Unknown tile {}", tile)
            }
            VaultError::ExactlyOneEntrance => {
                write!(f, "Vault must have exactly one entrance")
            }
            VaultError::NoPath => {
                write!(f, "Did not find a path to all keys")
            }
        }
    }
}
