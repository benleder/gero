use gero::models::{RecruitmentChallenge, LoreQuestion};

#[test]
fn score_increments_and_completes() {
    let questions = vec![LoreQuestion {
        question: "Q".into(),
        options: vec!["A".into(), "B".into()],
        correct_answer_index: 1,
        explanation: String::new(),
    }];
    let mut challenge = RecruitmentChallenge {
        unit_name: "recruit".into(),
        questions,
        required_correct_answers: 1,
        player_score: 0,
        is_completed: false,
    };

    assert!(challenge.present_question(0).is_some());
    let correct = challenge.record_answer(0, 1);
    assert!(correct);
    assert_eq!(challenge.player_score, 1);
    assert!(challenge.is_completed);
}

#[test]
fn spawns_unit_when_score_met() {
    let questions = vec![LoreQuestion {
        question: "Q".into(),
        options: vec!["A".into()],
        correct_answer_index: 0,
        explanation: String::new(),
    }];
    let mut challenge = RecruitmentChallenge {
        unit_name: "hero".into(),
        questions,
        required_correct_answers: 1,
        player_score: 0,
        is_completed: false,
    };

    challenge.record_answer(0, 0);
    let unit = challenge.spawn_unit();
    assert!(unit.is_some());
    let unit = unit.unwrap();
    assert_eq!(unit.name, "hero");
}
