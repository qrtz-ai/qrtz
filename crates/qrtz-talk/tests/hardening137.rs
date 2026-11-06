//! ADR 0253 (echo cycle 137): a run of 3+ consecutive "on" or "like" (any case) collapses to the
//! first copy. Doubles stay; "are" triple+ from ADR 0251 still collapses; never across commas.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn triple_on_and_like_stutter_collapses() {
    for (raw, want) in [
        ("on on On the replay modal", "On the replay modal."),
        (
            "like on on On the replay modal please",
            "Like on the replay modal please.",
        ),
        ("turn it on on on Friday please", "Turn it on Friday please."),
        (
            "Can you add a toggle for each like on on on the modal",
            "Can you add a toggle for each like on the modal?",
        ),
        ("like like like to ping an API", "Like to ping an API."),
        (
            "easiest for you to like like uh like to ping",
            "Easiest for you to like to ping.",
        ),
        (
            "she was like LIKE like totally shocked today",
            "She was like totally shocked today.",
        ),
        (
            "Please make it like like like a smooth fade",
            "Please make it like a smooth fade.",
        ),
        // Prior: ADR 0251 triple are; STUTTERS triple the; ADR 0248 Cap contraction.
        (
            "Are are are we always fetching on hover",
            "Are we always fetching on hover?",
        ),
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
fn triple_on_like_0253_refusals_stay() {
    for (raw, want) in [
        ("turn it on on Friday please", "Turn it on on Friday please."),
        (
            "focus on On boarding the new hires",
            "Focus on On boarding the new hires.",
        ),
        (
            "I was like like whatever about it",
            "I was like like whatever about it.",
        ),
        (
            "like Like it or not we still ship today",
            "Like Like it or not we still ship today.",
        ),
        (
            "like a like a smooth fade across the board",
            "Like a like a smooth fade across the board.",
        ),
        (
            "on, on the other hand we wait today",
            "On, on the other hand we wait today.",
        ),
        (
            "like, like I said before today",
            "Like, like I said before today.",
        ),
        (
            "why Why did we make a new Thing",
            "Why Why did we make a new Thing.",
        ),
        (
            "you are Are you sure about that",
            "You are Are you sure about that.",
        ),
        (
            "The results are are interesting today",
            "The results are are interesting today.",
        ),
        ("is is that correct though", "Is is that correct though."),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
