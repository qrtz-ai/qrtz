//! ADR 0261 (echo cycle 154): an all-lowercase "than than" stutter collapses to one "than".
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn lowercase_than_stutter_collapses() {
    for (raw, want) in [
        // Real Mac history shape (dev).
        (
            "of a symbolic representation than than an actual yeah, if it is an array",
            "Of a symbolic representation than an actual yeah, if it is an array.",
        ),
        (
            "It's more of a symbolic representation than than an actual list today",
            "It's more of a symbolic representation than an actual list today.",
        ),
        (
            "This is way faster than than the old build today",
            "This is way faster than the old build today.",
        ),
        (
            "I would rather walk than than drive to the office today",
            "I would rather walk than drive to the office today.",
        ),
        (
            "It costs more than than than we planned today",
            "It costs more than we planned today.",
        ),
        (
            "It is more than\tthan before today",
            "It is more than before today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn cap_comma_or_other_words_keep_than() {
    for raw in [
        // A Cap copy may be a name.
        "It is faster than Than Shwe expected today",
        "Than than is not a word today",
        "I said than THAN on purpose today",
        // A comma is a clause boundary as often as a stutter.
        "It is bigger than, than the other one today",
        // Not a repeat.
        "It's more than thanks today",
        "It is more then than before today",
    ] {
        let want = format!("{raw}.");
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
