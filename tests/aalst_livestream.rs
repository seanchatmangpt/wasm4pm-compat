use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

struct TransitionEvent {
    transition_id: &'static str,
    timestamp: u64,
    tokens_consumed: Vec<&'static str>,
    tokens_produced: Vec<&'static str>,
}

impl TransitionEvent {
    fn to_json(&self) -> String {
        let consumed = self
            .tokens_consumed
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");
        let produced = self
            .tokens_produced
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "{{\"transition_id\": \"{}\", \"timestamp\": {}, \"tokens_consumed\": [{}], \"tokens_produced\": [{}]}}",
            self.transition_id, self.timestamp, consumed, produced
        )
    }
}

pub fn simulate_petri_net_replay() {
    let transitions = vec![
        TransitionEvent {
            transition_id: "t1",
            timestamp: 0,
            tokens_consumed: vec!["p1"],
            tokens_produced: vec!["p2", "p3"],
        },
        TransitionEvent {
            transition_id: "t2",
            timestamp: 10,
            tokens_consumed: vec!["p2"],
            tokens_produced: vec!["p4"],
        },
        TransitionEvent {
            transition_id: "t3",
            timestamp: 20,
            tokens_consumed: vec!["p3"],
            tokens_produced: vec!["p5"],
        },
        TransitionEvent {
            transition_id: "t4",
            timestamp: 30,
            tokens_consumed: vec!["p4", "p5"],
            tokens_produced: vec!["p6"],
        },
    ];

    let start = Instant::now();

    for event in transitions {
        let now = start.elapsed().as_millis() as u64;
        if event.timestamp > now {
            thread::sleep(Duration::from_millis(event.timestamp - now));
        }

        println!("{}", event.to_json());
        io::stdout().flush().unwrap();
    }
}

#[test]
fn test_aalst_livestream() {
    simulate_petri_net_replay();
}
