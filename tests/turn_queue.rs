use gero::combat::TurnQueue;
use std::collections::VecDeque;

#[test]
fn turn_order_cycles_and_tracks_current_unit() {
    let mut queue = TurnQueue::new();
    queue.add_unit("u1".into());
    queue.add_unit("u2".into());

    let first = queue.next_turn();
    assert_eq!(first.as_deref(), Some("u1"));
    assert_eq!(queue.current_unit_id.as_deref(), Some("u1"));
    assert_eq!(
        queue.initiative,
        VecDeque::from(vec!["u2".to_string(), "u1".to_string()])
    );

    let second = queue.next_turn();
    assert_eq!(second.as_deref(), Some("u2"));
    assert_eq!(queue.current_unit_id.as_deref(), Some("u2"));
    assert_eq!(
        queue.initiative,
        VecDeque::from(vec!["u1".to_string(), "u2".to_string()])
    );
}
