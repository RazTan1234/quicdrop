//! Core of quicdrop: the transfer protocol, shared by the CLI and the relay.

/// Version of the wire protocol. Bumped on every incompatible change, so two
/// peers can detect that they cannot talk to each other.
pub const PROTOCOL_VERSION: u16 = 1;

/// Returns true if a peer speaking `version` can talk to us.
pub fn is_compatible(version: u16) -> bool {
    version == PROTOCOL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_version_is_compatible() {
        assert!(is_compatible(PROTOCOL_VERSION));
    }

    #[test]
    fn different_version_is_not_compatible() {
        assert!(!is_compatible(PROTOCOL_VERSION + 1));
    }
}
