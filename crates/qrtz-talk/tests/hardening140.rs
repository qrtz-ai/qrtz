//! ADR 0256 (echo cycle 140): Cap-then-lower / both-lower restart phrases across a single period.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn period_gap_restarts_collapse() {
    for (raw, want) in [
        // Real Mac history case (both-lower across period).
        (
            "And then the output should be. should be a simple lightweight React animation today",
            "And then the output should be a simple lightweight React animation today.",
        ),
        (
            "Thank you. thank you for the review today",
            "Thank you for the review today.",
        ),
        (
            "Can you. can you host the prototype today",
            "Can you host the prototype today?",
        ),
        (
            "What's. what's the meeting pass code today",
            "What's the meeting pass code today?",
        ),
        ("Is that. is that one good today", "Is that one good today?"),
        (
            "You have to. you have to ship it today",
            "You have to ship it today.",
        ),
        (
            "How's it going. how's it going for you today",
            "How's it going for you today?",
        ),
        (
            "Let's just. let's just make the feature flag today",
            "Let's just make the feature flag today.",
        ),
        // Prior Cap-then-lower space / ellipsis still work.
        ("Is that is that one good today", "Is that one good today?"),
        (
            "Is that... is that one good for us now",
            "Is that one good for us now?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn period_gap_0256_refusals_stay() {
    for (raw, want) in [
        // Cap Cap across single period stays.
        (
            "I will go. I will go to the store today",
            "I will go. I will go to the store today.",
        ),
        (
            "I talk clearly. I talk clearly about that today",
            "I talk clearly. I talk clearly about that today.",
        ),
        // Space-only both-lower MW stays.
        (
            "like a like a simple path today",
            "Like a like a simple path today.",
        ),
        (
            "the starting deck the starting deck looks fine today",
            "The starting deck the starting deck looks fine today.",
        ),
        (
            "is that is that one good today",
            "Is that is that one good today.",
        ),
        // Cap Cap / discourse without apostrophe.
        (
            "Where where we put the snap summon today",
            "Where where we put the snap summon today.",
        ),
        (
            // continued_dot drops the period; restart_phrase must not then collapse "No no".
            "No. no just push it for us today please",
            "No no just push it for us today please.",
        ),
        // Prior ADR 0255 both-lower keep shape after sentence cap.
        (
            "you have to you have to ship it today",
            "You have to you have to ship it today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
