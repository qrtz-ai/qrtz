//! ADR 0251 (echo cycle 135): a run of 3+ consecutive "are" (any case) collapses to the first copy.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn triple_are_stutter_collapses() {
    for (raw, want) in [
        (
            "Are are are we always fetching on hover",
            "Are we always fetching on hover?",
        ),
        ("are are are we sure about this", "Are we sure about this?"),
        ("we are are are going tomorrow", "We are going tomorrow."),
        ("Are are are are we done", "Are we done"),
        ("Are ARE are we ready yet", "Are we ready yet?"),
        (
            "they are Are ARE coming over later",
            "They are coming over later.",
        ),
        ("so are are are the results in", "So are the results in."),
        (
            "And then are are are we shipping tonight",
            "And then are we shipping tonight.",
        ),
        // Prior: STUTTERS triple "the" still collapses; ADR 0248 Cap contraction still collapses.
        ("the the the cat sat on the mat", "The cat sat on the mat."),
        (
            "Yeah, it's still flying in, it's It's not correct",
            "Yeah, it's still flying in, it's not correct.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn triple_are_0251_refusals_stay() {
    for (raw, want) in [
        (
            "you are Are you sure about that",
            "You are Are you sure about that.",
        ),
        (
            "they are Are they coming today",
            "They are Are they coming today.",
        ),
        ("What are Are we doing next", "What are Are we doing next."),
        ("there are Are there any left", "There are Are there any left."),
        (
            "The results are are interesting today",
            "The results are are interesting today.",
        ),
        ("Are Are we ready to ship", "Are Are we ready to ship."),
        ("turn it on on Friday please", "Turn it on on Friday please."),
        ("is is that correct though", "Is is that correct though."),
        // Comma-separated are stays (never collapse across commas).
        ("we are, are we sure", "We are, are we sure."),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
