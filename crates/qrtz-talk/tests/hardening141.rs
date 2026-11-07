//! ADR 0257 (echo cycle 141): both-lower contraction-headed multi-word restart (spaces/ellipsis).
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn contraction_headed_both_lower_mw_collapses() {
    for (raw, want) in [
        // Real Mac history case (prod).
        (
            "Does a set timeout return a variable or what's it what's it return today",
            "Does a set timeout return a variable or what's it return today.",
        ),
        ("what's it what's it return today", "What's it return today?"),
        (
            "don't know don't know the answer today",
            "Don't know the answer today.",
        ),
        (
            "how's it how's it going for you today",
            "How's it going for you today?",
        ),
        (
            "where's the where's the localhost link today",
            "Where's the localhost link today?",
        ),
        (
            "don't really know don't really know today",
            "Don't really know today.",
        ),
        (
            "can't we just have can't we just have the flag today",
            "Can't we just have the flag today.",
        ),
        ("what's it... what's it return today", "What's it return today?"),
        // Prior Cap-then-lower / period-gap still work.
        ("Is that is that one good today", "Is that one good today?"),
        (
            "And then the output should be. should be a simple path today",
            "And then the output should be a simple path today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn contraction_headed_0257_refusals_stay() {
    for (raw, want) in [
        // Space-only both-lower without apostrophe stays.
        (
            "like a like a simple path today",
            "Like a like a simple path today.",
        ),
        (
            "for the for the team meeting today",
            "For the for the team meeting today.",
        ),
        (
            "on the on the replay modal today",
            "On the on the replay modal today.",
        ),
        (
            "is that is that one good today",
            "Is that is that one good today.",
        ),
        ("as a as a simple path today", "As a as a simple path today."),
        ("in the in the morning today", "In the in the morning today."),
        // Both-lower one-word contractions stay (ADR 0248 / 0254).
        (
            "it's it's ready for review today",
            "It's it's ready for review today.",
        ),
        (
            "don't don't push that change today",
            "Don't don't push that change today.",
        ),
        (
            "we're we're late to the standup today",
            "We're we're late to the standup today.",
        ),
        // Content double.
        (
            "had had enough of that already today",
            "Had had enough of that already today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
