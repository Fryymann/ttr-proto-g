use ttrpg_protocol::SceneCommand;

#[derive(Debug, Clone)]
pub struct CommandEnvelope {
    pub actor_id: String,
    pub command: SceneCommand,
    pub timestamp_ms: u64,
    pub sequence_id: u64,
}

// Implement Ord for deterministic sorting: timestamp first, then actor_id tie-breaker.
impl PartialEq for CommandEnvelope {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp_ms == other.timestamp_ms
            && self.actor_id == other.actor_id
            && self.sequence_id == other.sequence_id
    }
}

impl Eq for CommandEnvelope {}

impl PartialOrd for CommandEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CommandEnvelope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp_ms
            .cmp(&other.timestamp_ms)
            .then_with(|| self.actor_id.cmp(&other.actor_id))
            .then_with(|| self.sequence_id.cmp(&other.sequence_id))
    }
}

#[derive(Clone)]
pub struct SceneCommandQueue {
    pending: Vec<CommandEnvelope>,
    next_sequence: u64,
}

impl SceneCommandQueue {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            next_sequence: 0,
        }
    }

    pub fn push(&mut self, mut envelope: CommandEnvelope) {
        envelope.sequence_id = self.next_sequence;
        self.next_sequence += 1;
        self.pending.push(envelope);
    }

    /// Drains the queue, returning commands in deterministic order.
    pub fn drain_deterministic(&mut self) -> Vec<CommandEnvelope> {
        self.pending.sort();
        std::mem::take(&mut self.pending)
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttrpg_protocol::ScenePosition;

    #[test]
    fn test_deterministic_ordering() {
        let mut queue = SceneCommandQueue::new();
        
        let cmd1 = CommandEnvelope {
            actor_id: "actor-b".to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 1, y: 1 } },
            timestamp_ms: 100,
            sequence_id: 0,
        };
        let cmd2 = CommandEnvelope {
            actor_id: "actor-a".to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 2, y: 2 } },
            timestamp_ms: 100,
            sequence_id: 0,
        };
        let cmd3 = CommandEnvelope {
            actor_id: "actor-c".to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 3, y: 3 } },
            timestamp_ms: 50,
            sequence_id: 0,
        };

        // Push in "random" order
        queue.push(cmd1.clone());
        queue.push(cmd2.clone());
        queue.push(cmd3.clone());

        let drained = queue.drain_deterministic();

        assert_eq!(drained.len(), 3);
        // Should be sorted by timestamp first
        assert_eq!(drained[0].timestamp_ms, 50);
        // Then by actor_id for same timestamp
        assert_eq!(drained[1].actor_id, "actor-a");
        assert_eq!(drained[2].actor_id, "actor-b");
    }
}
