//! ADR 0259 (echo cycle 143): Cap Cap / Cap-second multi-word across a single period collapses
//! when the first copy contains a verbal contraction.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn cap_cap_period_mw_verbal_collapses() {
    for (raw, want) in [
        // Real Mac history shape (dev).
        (
            "I don't. I don't know how to do all this stuff today",
            "I don't know how to do all this stuff today.",
        ),
        (
            "you can't. You can't ship that without tests today",
            "You can't ship that without tests today.",
        ),
        (
            "Don't really. Don't really like this approach today",
            "Don't really like this approach today.",
        ),
        (
            "What's the. What's the meeting pass code today",
            "What's the meeting pass code today?",
        ),
        (
            "I don't really. I don't really like this idea today",
            "I don't really like this idea today.",
        ),
        (
            "We can't. We can't ship without the tests today",
            "We can't ship without the tests today.",
        ),
        (
            "I wouldn't. I wouldn't take that path today",
            "I wouldn't take that path today.",
        ),
        (
            "How's it. How's it going for the team today",
            "How's it going for the team today?",
        ),
        // Prior Cap Cap one-word period still works.
        (
            "Like, I'm. I'm a senior engineer now today",
            "Like, I'm a senior engineer now today.",
        ),
        // Prior Cap-then-lower period still works.
        (
            "And then the output should be. should be a simple path today",
            "And then the output should be a simple path today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn cap_cap_period_mw_0259_refusals_stay() {
    for (raw, want) in [
        // Cap Cap MW without verbal contraction across period.
        (
            "I will go. I will go tomorrow with the team",
            "I will go. I will go tomorrow with the team.",
        ),
        (
            "I talk. I talk like I know everything today",
            "I talk. I talk like I know everything today.",
        ),
        (
            "I talk clearly. I talk clearly about that today",
            "I talk clearly. I talk clearly about that today.",
        ),
        (
            "We should ship. We should ship the flag today",
            "We should ship. We should ship the flag today.",
        ),
        (
            "Please check. Please check the logs for us today",
            "Please check. Please check the logs for us today.",
        ),
        // Cap Cap one-word non-contraction / spaces still refused.
        (
            "So. So we should ship the flag today",
            "So. So we should ship the flag today.",
        ),
        (
            "I'm I'm a senior engineer on the team today",
            "I'm I'm a senior engineer on the team today.",
        ),
        (
            "I'm... I'm a senior engineer on the team today",
            "I'm... I'm a senior engineer on the team today.",
        ),
        // Names / possessives.
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
