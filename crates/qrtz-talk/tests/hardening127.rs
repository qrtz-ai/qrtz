//! ADR 0244 (echo cycle 127): a confirmation tag said as its own sentence ("…. Right" / "…. Correct") ends with "?".
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn confirmation_sentences_end_with_question_mark() {
    for (raw, want) in [
        (
            "Just to confirm, all of this will automatically update. Right",
            "Just to confirm, all of this will automatically update. Right?",
        ),
        (
            "Just to confirm, all of this will automatically update. Right.",
            "Just to confirm, all of this will automatically update. Right?",
        ),
        (
            "And then just these ones morph together. Right",
            "And then just these ones morph together. Right?",
        ),
        (
            "All of this will automatically update. Correct",
            "All of this will automatically update. Correct?",
        ),
        ("We should ship today. Right", "We should ship today. Right?"),
        (
            "That matches the design. Correct",
            "That matches the design. Correct?",
        ),
        (
            "The CSV is what you update. Right",
            "The CSV is what you update. Right?",
        ),
        (
            "You're editing the document I sent. Correct.",
            "You're editing the document I sent. Correct?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn confirmation_sentence_refusals_stay() {
    for (raw, want) in [
        ("Turn left. Right", "Turn left. Right."),
        ("Go up. Right", "Go up. Right."),
        ("Look north. Right", "Look north. Right."),
        ("Please turn left. Right.", "Please turn left. Right."),
        ("Face west. Right", "Face west. Right."),
        ("Step forward. Right", "Step forward. Right."),
        (
            "And then just these ones morph together. Right?",
            "And then just these ones morph together. Right?",
        ),
        ("Right", "Right"),
        ("Correct", "Correct"),
        ("Copy that. Right away", "Copy that. Right away."),
        ("Keep going back. Right", "Keep going back. Right."),
        ("Then left. Right", "Then left. Right."),
        ("I got it right.", "I got it right."),
        (
            "That answer is politically correct.",
            "That answer is politically correct.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
