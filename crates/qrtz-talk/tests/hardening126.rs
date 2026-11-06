//! ADR 0243 (echo cycle 126): an unmarked confirmation tag (", right" / ", correct") ends with "?".
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn confirmation_tags_end_with_question_mark() {
    for (raw, want) in [
        ("That works for me, right", "That works for me, right?"),
        (
            "The CSV is what you update, right",
            "The CSV is what you update, right?",
        ),
        (
            "You're editing the document I sent you, right",
            "You're editing the document I sent you, right?",
        ),
        (
            "There has to be a better way to load this, correct",
            "There has to be a better way to load this, correct?",
        ),
        ("You mean this one, correct", "You mean this one, correct?"),
        (
            "We should have CI on the release pipeline, right",
            "We should have CI on the release pipeline, right?",
        ),
        (
            "I would surely know about it, right",
            "I would surely know about it, right?",
        ),
        (
            "Make sure the tests pass, correct",
            "Make sure the tests pass, correct?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn confirmation_tag_refusals_stay() {
    for (raw, want) in [
        ("Please turn left, right", "Please turn left, right."),
        ("Go up, down, left, right", "Go up, down, left, right."),
        ("Look north, right", "Look north, right"),
        ("Copy that, right away", "Copy that, right away."),
        ("I got it right", "I got it right."),
        (
            "That answer is politically correct",
            "That answer is politically correct.",
        ),
        (
            "Just to confirm, all of this will automatically update. Right",
            "Just to confirm, all of this will automatically update. Right.",
        ),
        ("It's fine, right?", "It's fine, right?"),
        ("Am I right", "Am I right"),
        ("Keep right at the fork", "Keep right at the fork."),
        ("Yes, right", "Yes, right"),
        ("Then left, right", "Then left, right"),
        ("Put it on the right", "Put it on the right."),
        (
            "Mark the first answer wrong, the second correct",
            "Mark the first answer wrong, the second correct.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
