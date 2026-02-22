pub mod fallback;
pub mod state;
pub mod timer;

pub use fallback::{resolve_timeout_fallback, TimeoutFallbackAction, TimeoutFallbackError};
pub use state::{capture_participants, EncounterState, InitiativeScore};
pub use timer::{TurnTimerConfig, TurnTimerMarker};

pub fn deterministic_initiative_for_actor(actor_id: &str) -> InitiativeScore {
    let checksum = actor_id.bytes().fold(0_u32, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u32)
    });

    InitiativeScore {
        roll_total: (checksum % 20 + 1) as i32,
        dex_mod: ((checksum / 20) % 11) as i32 - 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_initiative_is_stable_for_same_actor() {
        let first = deterministic_initiative_for_actor("Alpha");
        let second = deterministic_initiative_for_actor("Alpha");
        assert_eq!(first, second);
    }
}
