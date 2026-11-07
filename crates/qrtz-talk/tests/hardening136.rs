//! ADR 0252 (echo cycle 136): Cap-restart phrases may have an ellipsis (`...` / `…`) between copies.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn ellipsis_cap_restart_collapses() {
    for (raw, want) in [
        (
            "But um I don't really... I don't really like this. What would you recommend?",
            "But I don't really like this. What would you recommend?",
        ),
        (
            "I don't really... I don't really like this",
            "I don't really like this.",
        ),
        (
            "I don't really… I don't really like this today",
            "I don't really like this today.",
        ),
        (
            "I don't... I don't know how to fix that",
            "I don't know how to fix that.",
        ),
        (
            "We should go... We should go now please",
            "We should go now please.",
        ),
        (
            "Wait for it... Wait for it to load first",
            "Wait for it to load first.",
        ),
        (
            "Can you... Can you host the prototype on a server",
            "Can you host the prototype on a server?",
        ),
        (
            "I don't know... I don't know how to do that UI",
            "I don't know how to do that UI.",
        ),
        (
            "we could just have... We could just have our configuration ready",
            "We could just have our configuration ready.",
        ),
        (
            "What are the... What are the deciding factors on both sides",
            "What are the deciding factors on both sides?",
        ),
        // Prior space-only Cap restart still collapses.
        (
            "Can you Can you host the prototype",
            "Can you host the prototype?",
        ),
        (
            "Are are are we always fetching on hover",
            "Are we always fetching on hover?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn ellipsis_cap_restart_0252_refusals_stay() {
    for (raw, want) in [
        (
            "I'll use it. It needs to work correctly today",
            "I'll use it. It needs to work correctly today.",
        ),
        (
            "I talk clearly. I talk clearly about this idea",
            "I talk clearly. I talk clearly about this idea.",
        ),
        (
            "new york... New York looks nice today",
            "New york... New York looks nice today.",
        ),
        (
            "scratch that... Scratch that for me please",
            "Scratch that... Scratch that for me please.",
        ),
        (
            "like a like a smooth fade across the board",
            "Like a like a smooth fade across the board.",
        ),
        (
            "I was thinking... she was leaving early today",
            "I was thinking... She was leaving early today.",
        ),
        (
            "Wait... Wait a minute please now",
            "Wait... Wait a minute please now.",
        ),
        (
            "The results are are interesting... today for us",
            "The results are are interesting... Today for us.",
        ),
        // Space-only prior keeps still hold.
        (
            "you are Are you sure about that",
            "You are Are you sure about that.",
        ),
        (
            "new york New York looks nice today",
            "New york New York looks nice today.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
