//! ADR 0258 (echo cycle 142): Cap Cap one-word verbal contraction across a single period.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn cap_cap_period_contraction_collapses() {
    for (raw, want) in [
        // Real Mac history shape (dev).
        (
            "Like, I'm. I'm a senior engineer now today",
            "Like, I'm a senior engineer now today.",
        ),
        (
            "It's. It's the top two things today",
            "It's the top two things today.",
        ),
        (
            "Don't. Don't push that change today",
            "Don't push that change today.",
        ),
        (
            "What's. What's the meeting pass code today",
            "What's the meeting pass code today?",
        ),
        (
            "Can't. Can't we just ship the flag today",
            "Can't we just ship the flag today.",
        ),
        (
            "We're. We're late to the standup today",
            "We're late to the standup today.",
        ),
        (
            "That's. That's not true about the release today",
            "That's not true about the release today.",
        ),
        (
            "How's. How's it going for the team today",
            "How's it going for the team today?",
        ),
        // Prior Cap-then-lower period still works.
        (
            "And then the output should be. should be a simple path today",
            "And then the output should be a simple path today.",
        ),
        // Prior Cap-then-lower contraction still works.
        (
            "What's what's the meeting pass code today",
            "What's the meeting pass code today?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn cap_cap_period_0258_refusals_stay() {
    for (raw, want) in [
        // Cap Cap non-contraction across period.
        (
            "I will go. I will go tomorrow with the team",
            "I will go. I will go tomorrow with the team.",
        ),
        (
            "So. So we should ship the flag today",
            "So. So we should ship the flag today.",
        ),
        (
            "Yes. Yes I can do that with the team today",
            "Yes. Yes I can do that with the team today.",
        ),
        // ADR 0260: Cap Cap "I. I" across a period now collapses (moved to hardening145).
        (
            "Where. Where is the release for the team today",
            "Where. Where is the release for the team today?",
        ),
        // Cap Cap verbal contraction across spaces / ellipsis stays.
        (
            "I'm I'm a senior engineer on the team today",
            "I'm I'm a senior engineer on the team today.",
        ),
        (
            "I'm... I'm a senior engineer on the team today",
            "I'm... I'm a senior engineer on the team today.",
        ),
        (
            "It's It's ready for review with the team today",
            "It's It's ready for review with the team today.",
        ),
        // Name / possessive Cap Cap across period stays.
        (
            "Talk to O'Brien. O'Brien said yes about the release today",
            "Talk to O'Brien. O'Brien said yes about the release today.",
        ),
        (
            "Bob's. Bob's car is ready for the drive today",
            "Bob's. Bob's car is ready for the drive today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
