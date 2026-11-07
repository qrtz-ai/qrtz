//! ADR 0260 (echo cycle 145): Cap Cap pronoun "I" across a single period collapses.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn cap_cap_period_i_collapses() {
    for (raw, want) in [
        // Real Mac history shape (dev).
        (
            "But I. I just feel like a moron today",
            "But I just feel like a moron today.",
        ),
        (
            "I. I think that works for the team today",
            "I think that works for the team today.",
        ),
        (
            "Yeah I. I agree with the privacy review today",
            "Yeah I agree with the privacy review today.",
        ),
        (
            "Oh yeah, I. I wasn't even thinking about that today",
            "Oh yeah, I wasn't even thinking about that today.",
        ),
        (
            "Next plus plus, I. I assume here this is something today",
            "Next plus plus, I assume here this is something today.",
        ),
        (
            "Like I. I don't know how to do all this stuff today",
            "Like I don't know how to do all this stuff today.",
        ),
        (
            "Because I. I absolutely forget all of those things today",
            "Because I absolutely forget all of those things today.",
        ),
        (
            "Sorry I. I actually want the card mechanics today",
            "Sorry I actually want the card mechanics today.",
        ),
        // Prior Cap Cap one-word verbal period still works.
        (
            "Like, I'm. I'm a senior engineer now today",
            "Like, I'm a senior engineer now today.",
        ),
        // Prior Cap Cap MW verbal period still works.
        (
            "I don't. I don't know how to do all this stuff today",
            "I don't know how to do all this stuff today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn cap_cap_period_i_0260_refusals_stay() {
    for (raw, want) in [
        // Cap Cap non-I STUTTERS / discourse across period stay.
        (
            "So. So we should ship the flag today",
            "So. So we should ship the flag today.",
        ),
        (
            "Yes. Yes I can do that with the team today",
            "Yes. Yes I can do that with the team today.",
        ),
        (
            "Where. Where is the release for the team today",
            "Where. Where is the release for the team today?",
        ),
        (
            "What. What is the release for the team today",
            "What. What is the release for the team today?",
        ),
        (
            "The. The plan for the team is ready today",
            "The. The plan for the team is ready today.",
        ),
        (
            "Will. Will you join the standup with us today",
            "Will. Will you join the standup with us today?",
        ),
        (
            "We. We should ship the flag with the team today",
            "We. We should ship the flag with the team today.",
        ),
        (
            "A. A few days later we ship the flag today",
            "A. A few days later we ship the flag today.",
        ),
        // Cap Cap I across ellipsis stays (period only).
        (
            "I... I think that works for the team today",
            "I... I think that works for the team today.",
        ),
        // Cap Cap MW without verbal stays.
        (
            "I talk. I talk like I know everything today",
            "I talk. I talk like I know everything today.",
        ),
        // Cap Cap verbal contraction across spaces stays.
        (
            "I'm I'm a senior engineer on the team today",
            "I'm I'm a senior engineer on the team today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
